# About

rs-rpgmvp is a commandline tool(CLI) for decrypting rpg maker(.rpgmvp) files.

# Installing
Install the latest rust toolset from rustup:  
[rustup.rs](https://rustup.rs/)


Run the below commands:
```
cargo install --git https://github.com/Lonestar137/rs-rpgmvp.git
```

Ensure that the Cargo bin(`~/.cargo/bin` on Linux) is on your users PATH.

# How to use

## CLI args

```bash
CLI tool for decrypting RPGMVP tools

Usage: rs-mpgv [OPTIONS] --decryption-key <DECRYPTION_KEY> <--basepath <BASEPATH>|--files <FILES>>

Options:
  -b, --basepath <BASEPATH>
          Directory to glob rpgmvp files from
  -f, --files <FILES>
          Takes a list of files to decrypt
  -e, --extension <EXTENSION>
          File extension of files to decrypt [default: rpgmvp]
  -o, --output <OUTPUT>
          Folder to output processed files into
  -d, --decryption-key <DECRYPTION_KEY>
          Key used to decrypt files. Note: Found in www/data/System.json
  -h, --help
          Print help
  -V, --version
          Print version
  
```

Decrypt an entire directory of files:  
`rs-rpgmvp --decryption-key {key from www/data/System.json} --basepath {folder/with/rpgmvp/files}`

Decrypt specific files:  
`rs-rpgmvp --decryption-key {key from www/data/System.json} --files {path/file1} {path/file2}`

Save decrypted files to a custom location(Preserves folder structure):  
`rs-rpgmvp --decryption-key {key from www/data/System.json} --basepath {folder/with/rpgmvp/files} --output /tmp/DecryptedFilesFolder/`


# Its missing something

If you have an idea of how the tool can be better feel free to raise an issue 
or pull request!

# Credit

For providing a working implementation of the spec:  
[Petschko RPGMVP Decrypter](https://github.com/Petschko/RPG-Maker-MV-Decrypter)
