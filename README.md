# gmfmod_decrypt
Simple program for decrypting soundfiles protected with GMFMOD's built in protection.

icuurd12b12's GMFMOD (FMODEx for GameMaker) has the function `FMODEncryptFile` for protecting files with a SHA1 hash as a xor key.
Unfortunately the design is a bit stupid, because the password hash is appended at EOF followed by `FMODSIMPLEPW`, however you need the hash of the password in reverse as well so you do need to crack it.

# Usage
```
$ gmfmod_decrypt sound.wav
Password: very_secure_password
```

```
$ gmfmod_decrypt very_secure_password sound.wav
```

```
$ gmfmod_decrypt very_secure_password sound1.wav sound2.wav sound3.wav ...
```

```
$ gmfmod_decrypt very_secure_password *.wav
```
