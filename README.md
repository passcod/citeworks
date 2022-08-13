# Citeworks

- CSL types/serde: https://docs.rs/citeworks-csl
- CFF types/serde: https://docs.rs/citeworks-cff
- CLI tools:
  - install with `cargo binstall citeworks-cli`
  - `csl2cff`: converts CSL-JSON to CFF references

Install `cargo binstall citeworks-cli`.

### csl2cff

```console
$ csl2cff bibli.json
- type: article
  authors:
  - family-names: Vaidya
    given-names: Nina
  - family-names: Solgaard
    given-names: Olav
  abstract: Complex optical devices including aspherical focusing mirrors, solar concentrator
    arrays, and immersion lenses were 3D printed using commercial technology and experimentally
    demonstrated by evaluating surface roughness and shape. The as-printed surfaces
    had surface roughness on the order of tens of microns. To improve this unacceptable
    surface quality for creating optics, a polymer smoothing technique was developed.
    Atomic force microscopy and optical profilometry showed that the smoothing technique
    reduced the surface roughness to a few nanometers, consistent with the requirements
    of high-quality optics, while tests of optical functionality demonstrated that
    the overall shapes were maintained so that near theoretically predicted operation
    was achieved. The optical surface smoothing technique is a promising approach
    towards using 3D printing as a flexible tool for prototyping and fabrication of
    miniaturized high-quality optics.
  collection-title: Microsystems & Nanoengineering
  copyright: 2018 The Author(s)
  database: www.nature.com
  date-accessed: 2022-08-12
  doi: 10.1038/s41378-018-0015-4
  start: 1
  end: 8
  issn: 2055-7434
  issue: '1'
  issue-date: 2018-07-16
  journal: Microsyst Nanoeng
  languages:
  - en
  title: 3D printed optics with nanometer scale surface roughness
  url: https://www.nature.com/articles/s41378-018-0015-4
  volume: 4
```

Append to `CITATION.cff`'s `references`:

```console
$ csl2cff bibli.json --insert CITATION.cff
```

Replace the whole `references` list:

```console
$ csl2cff bibli.json --replace CITATION.cff
```

## Copyright

Félix Saparelli (:passcod)

License: Apache 2.0.
