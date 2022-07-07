# Patate chaude server

---

## Groupe 1

 - Quentin MOLERO
 - Remy MACHAVOINE
 - Florian GUISTI
 - Cédric LEPROHON

## Options pour le serveur

 - `-h` : Display help message
 - `-c` : Complexity of the challenge, default value is `16`
 - `-p` : Port of the server, default value is `7878`
 - `-r` : Number of rounds to play, default is `100`
 - `-s` : Secret password to block connections and start the server, default is `1234`
 - `-t` : Max time in seconds of a round, default is `2`
 - `-v` : Version of the server

:heavy_exclamation_mark: **Attention** :heavy_exclamation_mark:

Pour démarrer le server, il est essentiel de mettre un dictionnaire de mots à disposition afin qu'il génère aléatoirement les phrases du challenge.
Pour cela il faut respecter la structure de fichier suivante :
```
/patate_chaude_server.exe
/res/
/res/dictionary.txt
```
