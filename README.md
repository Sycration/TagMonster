# TagMonster
[<img src="./icon.svg" width="100" />](./icon.svg)

TagMonster is a desktop program designed to automate some of the tedious and error-prone parts of archiving work. This project is developed as an internal tool for [GenEq UC Berkeley](https://cejce.berkeley.edu/geneq). It is in active development and unsuitable for production use.

To run it, you will need Git and a recent Rust compiler:
```shell
$ git clone https://github.com/Sycration/TagMonster
$ cd TagMonster
$ cargo run
```

In the Program Settings window, you will need to sign in to both the Box and Google API using an API key/secret pair.

## Roadmap
- [x] UI design
- [x] Box API integration
- [x] Google API integration
- [x] Automated file type analysis
- [x] Incomplete spreadsheet generation
- [x] Offline folder support
- [x] Internal file browser
    - [ ] Better file browser - Custom widget or [this library?](https://github.com/EmmanuelDodoo/modav_widgets)
- [ ] Tagging and notes
- [x] Internal file viewer
- [x] Local spreadsheet export
- [ ] Manual & documentation
- [ ] Packaged and signed distribution
    - [ ] Windows
    - [ ] MacOS
        - [ ] App Store
    - [ ] Linux
- [ ] API platform verification
- [ ] Additional APIs
    - [ ] Google Drive source
    - [ ] File server source
        - [ ] FTP
        - [ ] WebDAV
        - [ ] ...
    - [ ] S3 object storage source

