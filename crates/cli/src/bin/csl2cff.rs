use std::{
	fs::File,
	path::{Path, PathBuf},
	str::FromStr,
};

use citeworks_cff::{
	from_reader as cff_from_reader,
	identifiers::Identifier,
	names::{EntityName, Name as CffName, PersonName},
	references::{RefType, Reference},
	to_writer, Cff, Date as CffDate,
};
use citeworks_csl::{
	dates::Date as CslDate, from_reader as csl_from_reader, items::ItemType,
	names::Name as CslName, ordinaries::OrdinaryValue, Item,
};
use clap::Parser;
use miette::{IntoDiagnostic, Result};
use url::Url;

#[derive(Debug, Parser)]
#[clap(author, about, version)]
struct Args {
	/// CSL-JSON file or - to read STDIN
	input: PathBuf,

	/// Append bibliography from CSL to references section of target CFF file
	#[clap(long, value_name = "TARGET")]
	insert: Option<PathBuf>,

	/// Replace references section of target CFF file with CSL bibliography
	#[clap(long, value_name = "TARGET")]
	replace: Option<PathBuf>,
}

fn main() -> Result<()> {
	let args = Args::parse();

	let csl = if args.input.to_str() == Some("-") {
		let stdin = std::io::stdin();
		csl_from_reader(stdin).into_diagnostic()?
	} else {
		let file = File::open(args.input).into_diagnostic()?;
		csl_from_reader(file).into_diagnostic()?
	};

	let mut refs = Vec::with_capacity(csl.len());
	for item in csl {
		refs.push(convert_ref(item)?);
	}

	if let Some(target) = args.replace {
		let mut cff = read_cff(&target)?;
		cff.references = refs;
		write_cff(&target, &cff)?;
	} else if let Some(target) = args.insert {
		let mut cff = read_cff(&target)?;
		cff.references.extend(refs);
		write_cff(&target, &cff)?;
	} else {
		print_references(refs)?;
	}

	Ok(())
}

fn read_cff(file: &Path) -> Result<Cff> {
	let file = File::open(file).into_diagnostic()?;
	cff_from_reader(file).into_diagnostic()
}

fn write_cff(target: &Path, cff: &Cff) -> Result<()> {
	let file = File::create(target).into_diagnostic()?;
	to_writer(file, cff).into_diagnostic()
}

fn print_references(refs: Vec<Reference>) -> Result<()> {
	let stdout = std::io::stdout();
	serde_yaml::to_writer(
		stdout,
		&serde_yaml::Value::Sequence(
			refs.into_iter()
				.map(|r| serde_yaml::to_value(r).unwrap())
				.collect(),
		),
	)
	.into_diagnostic()
}

fn convert_ref(item: Item) -> Result<Reference> {
	Ok(Reference {
		work_type: convert_type(item.item_type),
		authors: convert_authors(item.author.into_iter().chain(item.contributor.into_iter())),
		abbreviation: ov_string(item.title_short),
		abstract_text: ov_string(item.abstract_text),
		collection_title: ov_string(item.container_title),
		copyright: ov_string(item.rights).or_else(|| ov_string(item.license)),
		database: ov_string(item.source),
		date_accessed: convert_date(item.accessed),
		date_published: convert_date(item.published),
		doi: ov_string(item.doi),
		start: page_start(ov_string(item.page.clone())),
		end: page_end(ov_string(item.page.clone())),
		identifiers: extra_idents(ov_string(item.eissn), ov_string(item.issnl)),
		issn: ov_string(item.issn),
		issue: ov_string(item.issue),
		issue_date: convert_date(item.issued).map(|d| d.to_string()),
		journal: ov_string(item.journal_abbrevation),
		keywords: ov_string(item.category).map_or_else(|| Vec::new(), |c| vec![c]),
		languages: ov_string(item.language).map_or_else(|| Vec::new(), |c| vec![c]),
		notes: ov_string(item.note),
		title: ov_string(item.title),
		url: ov_string(item.url).and_then(|u| match Url::parse(&u) {
			Ok(url) => Some(url),
			Err(err) => {
				eprintln!("WARNING: could not parse URL {u:?}\n{err}");
				None
			}
		}),
		volume: ov_string(item.volume).and_then(|v| match u64::from_str(&v) {
			Ok(vol) => Some(vol),
			Err(err) => {
				eprintln!("WARNING: could not parse volume {v:?} as number\n{err}");
				None
			}
		}),
		..Default::default()
	})
}

fn convert_type(item_type: ItemType) -> RefType {
	match item_type {
		ItemType::Article => RefType::Article,
		ItemType::ArticleJournal => RefType::Article,
		ItemType::ArticleMagazine => RefType::MagazineArticle,
		ItemType::ArticleNewspaper => RefType::NewspaperArticle,
		ItemType::Bill => RefType::Bill,
		ItemType::Book => RefType::Book,
		ItemType::Broadcast => RefType::Generic,
		ItemType::Chapter => RefType::Book,
		ItemType::Classic => RefType::Generic,
		ItemType::Collection => RefType::Generic,
		ItemType::Dataset => RefType::Data,
		ItemType::Document => RefType::Generic,
		ItemType::Entry => RefType::Generic,
		ItemType::EntryDictionary => RefType::Dictionary,
		ItemType::EntryEncyclopedia => RefType::Encyclopedia,
		ItemType::Figure => RefType::Generic,
		ItemType::Graphic => RefType::Generic,
		ItemType::Hearing => RefType::Hearing,
		ItemType::Interview => RefType::Generic,
		ItemType::LegalCase => RefType::LegalCase,
		ItemType::Legislation => RefType::GovernmentDocument,
		ItemType::Manuscript => RefType::Generic,
		ItemType::Map => RefType::Map,
		ItemType::MotionPicture => RefType::Video,
		ItemType::MusicalScore => RefType::Music,
		ItemType::Pamphlet => RefType::Pamphlet,
		ItemType::PaperConference => RefType::ConferencePaper,
		ItemType::Patent => RefType::Patent,
		ItemType::Performance => RefType::Generic,
		ItemType::Periodical => RefType::Generic,
		ItemType::PersonalCommunication => RefType::PersonalCommunication,
		ItemType::Post => RefType::Blog,
		ItemType::PostWeblog => RefType::Blog,
		ItemType::Regulation => RefType::Statute,
		ItemType::Report => RefType::Report,
		ItemType::Review => RefType::Generic,
		ItemType::ReviewBook => RefType::Generic,
		ItemType::Software => RefType::Software,
		ItemType::Song => RefType::Music,
		ItemType::Speech => RefType::SoundRecording,
		ItemType::Standard => RefType::Standard,
		ItemType::Thesis => RefType::Thesis,
		ItemType::Treaty => RefType::GovernmentDocument,
		ItemType::Webpage => RefType::Website,
		ItemType::Gazette => RefType::Generic,
		ItemType::Video => RefType::Video,
		ItemType::LegalCommentary => RefType::Generic,
	}
}

fn convert_authors(csl: impl Iterator<Item = CslName>) -> Vec<CffName> {
	let mut authors: Vec<_> = csl.map(convert_name).collect();
	if authors.is_empty() {
		authors.push(CffName::Anonymous);
	}
	authors
}

fn convert_name(csl_name: CslName) -> CffName {
	if csl_name.family.is_some() || csl_name.given.is_some() {
		CffName::Person(PersonName {
			family_names: csl_name.family,
			given_names: csl_name.given,
			name_particle: csl_name.non_dropping_particle,
			name_suffix: csl_name.suffix,
			..Default::default()
		})
	} else if csl_name.literal.is_some() {
		CffName::Entity(EntityName {
			name: csl_name.literal,
			..Default::default()
		})
	} else {
		eprintln!("WARNING: a name could not be converted, using debug repr");
		CffName::Entity(EntityName {
			name: Some(format!("{:?}", csl_name)),
			..Default::default()
		})
	}
}

fn ov_string(ov: Option<OrdinaryValue>) -> Option<String> {
	ov.map(|v| v.to_string())
}

fn convert_date(date: Option<CslDate>) -> Option<CffDate> {
	match date {
		Some(CslDate::Single { date, .. }) => Some(CffDate {
			year: date.year,
			month: date.month,
			day: date.day,
		}),
		Some(CslDate::Range { start, .. }) => Some(CffDate {
			year: start.year,
			month: start.month,
			day: start.day,
		}),
		Some(other) => {
			eprintln!("WARNING: could not convert date {other:?}, do it manually");
			None
		}
		None => None,
	}
}

fn page_start(page: Option<String>) -> Option<u64> {
	if let Some(page) = page {
		if let Ok(single) = page.parse::<u64>() {
			Some(single)
		} else if let Some(start) = page.splitn(2, '-').next() {
			u64::from_str(start).ok()
		} else {
			None
		}
	} else {
		None
	}
}

fn page_end(page: Option<String>) -> Option<u64> {
	if let Some(page) = page {
		if let Ok(single) = page.parse::<u64>() {
			Some(single)
		} else if let Some(end) = page.splitn(2, '-').skip(1).next() {
			u64::from_str(end).ok()
		} else {
			None
		}
	} else {
		None
	}
}

fn extra_idents(eissn: Option<String>, issnl: Option<String>) -> Vec<Identifier> {
	let mut idents = Vec::new();
	if let Some(eissn) = eissn {
		idents.push(Identifier::Other {
			value: eissn,
			description: Some("EISSN".into()),
		});
	}
	if let Some(issnl) = issnl {
		idents.push(Identifier::Other {
			value: issnl,
			description: Some("ISSNL".into()),
		});
	}
	idents
}
