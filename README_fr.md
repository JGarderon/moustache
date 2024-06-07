# Moustache - l'application simple et rapide de préprocessing de texte

Note :
  - Vous lisez la version originale de ce README. Les autres versions sont des traductions réalisées par [ChatGPT](https://chatgpt.com) d'OpenAI.

## Préprocessing de texte, kesako ?

Le préprocessing (ou prétraitement en bon français) est l'action de produire un texte à partir d'un autre, à l'image de ce que peut faire un langage de programmation avec un module tels que [Jinja en Python](https://jinja.palletsprojects.com) - dont Moustache s'inspire pour la syntaxe.

Pour autant c'est une action de production limitée, généralement pour des fins sanitaires (éviter les erreurs ou les oublis) ou pour éviter [la répétition ingrate](https://fr.wikipedia.org/wiki/Ne_vous_r%C3%A9p%C3%A9tez_pas). Il ne s'agit pas de compilation, qui est une action modifiant [l'abstraction du code](https://fr.wikipedia.org/wiki/Compilateur).
Ce n'est pas non plus un langage destiné à être interprété pour donner un résultat au-delà de la production de texte. Le préprocessing n'est pas [Turing-complet](https://fr.wikipedia.org/wiki/Turing-complet) et Moustache pas davantage... même si, avec les extensions, on peut en réalitée faire énormément de choses comme avec un interpréteur de langage normal ! Mais ce n'est pas l'objectif : il faut mieux éviter.

__Alors que peut-on faire exactement un tel applicatif ? A quoi se destine t-il ?__ La réponse simple est : beaucoup, notamment pour la génération de contenus textuels statiques (HTML, Markdow, XML, etc.), qui repose sur quelques concepts simples :
  - _variabilité_ : définir des variables (qui n'ont ici qu'un type : du texte, comme dans un Shell type Bash),
  - _conditionnalité_ : rendre "optionnel" (conditionel) un bloc de texte ou d'instruction,
  - _inclusion_ : introduire un fichier ou un bloc de texte dans un autre,
  - _répétition_ : répéter autant souhaité un bloc de texte.

Cela peut sembler simple et sur le principe, ça l'est. Pourtant il y a une astuce magique pour rendre tout cela efficace et attractif : la récurcivité dans la production du texte. En somme c'est la capacité pour le texte à "ré-entrer" dans l'application qui l'a produit (de manière "transparente" pour l'utilisateur), afin de subir un nouveau traitement selon les mêmes règles... mais pas avec le même contenu. Il y a d'autres effets qui se produisent également, mais nous les verrons en détails après.

Bref le préprocessing de texte de Moustache c'est de la [macro](https://fr.wikipedia.org/wiki/Macro-d%C3%A9finition).

## Installation

Moustache est écrit en Rust et nécessite donc d'avoir [le compilateur installé](https://www.rust-lang.org/learn/get-started) sur la machine. Si vous souhaitez avoir une version très réduite de l'appliaction (moins d'une centaine de Ko), vous pouvez ajouter [l'utilitaire UPX](https://upx.github.io/) - mais ce n'est pas obligatoire.

La première étape est de cloner le dépôt sur lequel se trouve le code source :

```bash
git clone https://git.nothus.fr/produits/moustache.git
```

Exécutez ensuite dans votre terminal le fichier `install.sh` :

```bash
chmod +x ./install.sh # rendre le script exécutable
./install.sh # installation pour l'utilisateur courant
./install.sh +sudo # installation pour tout le système (nécessite des priviléges d'administrateur)
```

Voilà, c'est fini. Moustache est compilé et installé. Testons que tout va bien :

```bash
moustache --version
```

... Vous devriez avoir quelque chose qui ressemble à ça :

```
julien@debian:~/moustache$ moustache --version
v1.1.0
```

__Pour obtenir de l'aide à tout moment sur votre version précise, c'est facile : `moustache --help`__ Pour ceux qui souhaitent déjà mettre le nez dans le code : `cargo doc --open --all --all-features --document-private-items`.

__Pour les utilisateurs Windows__ la procédure est sensiblement la même. Tentez une compilation manuelle (`cargo build --release`) puis utilisez le résultat (fichier `target/release/moustache` depuis l'origine du dépôt).

## Les 3 délimiteurs possibles

Comme pour Jinja, il existe trois délimiteurs possibles :
  - `{# ... #}` pour les commentaires ("_comments_"), qui ne seront jamais affichés ni gardés dans la sortie (promis),
  - `{{ ... }}` pour les expressions ("_expressions_") - les variables qui seront effectivement remplacées par du texte en sortie,
  - `{% ... %}` pour les déclarations ("_statements_") - en quelque sorte les "ordres" ou consignes que vous indiquez.

Ce sont ces délimiteurs qui permettent, dans n'importe quel format source, de savoir ce qui est pour Moustache ou ce qui est du texte simple. C'est-à-dire qui ne subira _aucune_ transformation (littéralement aucune !).

Contrairement aux expressions ou aux commentaires, les déclarations ont plusieurs formats en fonction de leur usage. Généralement, elles entourent un bloc de texte qui peut lui-aussi contenir des délimiteurs.

A chaque fois qu'une variable est trouvée dans le contenu d'un délimileur, elle est remplacée par sa valeur textuelle.

Ainsi à chaque fois que Moustache prendre du contenu en entrée, si on l'autorise (via l'argument `-r`), il pourra le traiter en fonction des déclarations et expressions trouvés, puis remettre sur son entrée le contenu de sortie. S'il n'y a plusieurs rien à faire ou si l'on souhaite un seul traitement, la sortie est retournée. Cette notion de traitement peut être vue comme des étapes (`Step`) pour amener à la génération finale.

C'est en jonglant avec les délimiteurs et ce processus de traitements possiblement multiples, que l'on peut aboutir à des choses qui peuvent être très complexes.

__Attention :__
  - Un délimiteur ne peut pas en contenir un autre. Par exemple : `{# commentaire {{ mavar }} #}` est invalide alors que `{# commentaire #}{{ mavar }}` le sera.
  - Si une erreur est produite durant un traitement, ce dernier s'arrêtera avec un message explicatif et le document ne sera ni généré ni rendu.

### Délimiteur `{# ... #}` (commentaire)

Tout ce qui est entre `{#` et `#}` ne sera pas gardé. Jamais.

### Délimiteur `{{ ... }}` (expression)

C'est ce qui permet le remplacement d'une variable (appelé un symbole - `symbol`) par son équivalent textuel. Ce délimiteur ne fait rien d'autres !

Par facilité, on peut concaténer (sans espace ajouté) plusieurs variables ou texte ensemble :

```
{{ ma_variable_simple }}
{{ ma_variable_simple + " du texte" }}
{{ ma_variable_simple + " et " + une_autre_variable }}
```

Par soucis de lisibilité, la convention de Moustache est de mettre des espaces. Cependant ceci est parfaitement conforme :

```
{{ma_variable_simple}}
{{ma_variable_simple+" du texte"}}
{{ma_variable_simple+" et "+une_autre_variable}}
```
A vous de voir !

### Délimiteur `{% ... %}` (déclaration)

C'est là où la magie opère. Une déclaration dans Moustache peut être unitaire (unique) ou bordée (double : un début et une fin). On en trouve :
  - _déclaration "unitaire"_
    - `set` : créer ou définir une variable
    - `call` : appeler un bloc de texte à cet emplacement
    - `find` : trouver des fichiers, des dossiers (ou les deux) selon un gabarit
    - `include` : inclure un fichier à cet emplacement
    - `execute` : exécuter une extension (si la compilation l'embarque et que l'exécution l'a autorisée)
  - _déclarations "bordées"_
    - `if` (`endif`) : conditionne le texte contenu
    - `block` (`endblock`) : définit le texte contenu comme un bloc invoquable
    - `raw` (`endraw`) : n'exécute pas ce qui est dans le texte contenu
    - `for` (`endfor`) : boucle sur une "liste" (un item par ligne dans une chaîne de caractères) 

... Chacun a sa propre logique et une grammaire semblable.

#### Déclaration de définition d'une variable (`set`)

__Grammaire locale :__ 
  `{% set [symbol] = [text or symbol (+ text or symbol (+ ...))] (! if [empty | setted]) %}`

__Exemples :__
  - Définition d'une variable simple :
    ```
      {% set ma_variable = "mon texte" %}
      {% set ma_variable = une_autre_variable %}
    ```
    _La variable sera écrasée systématiquement si elle existe déjà._
  - Définition d'une variable simple avec concaténation :
    ```
      {% set ma_variable = "mon texte" + "autre chose" %}
      {% set ma_variable = une_autre_variable + une_autre %}
      {% set ma_variable = une_autre_variable + "du texte" %}
      {% set ma_variable = "du texte" + une_autre_variable %}
      ...
    ```
    _La variable sera écrasée systématiquement si elle existe déjà. On peut concaténer autant de textes ou de variables que souhaités, tant que le format avec le séparateur "+" est respecté._
  - Définition d'une variable simple (avec ou sans concaténation), _seulement si elle existe_
    ```
      {% set ma_variable = "mon texte" ! if setted %}
      {% set ma_variable = une_autre_variable ! if setted %}
      {% set ma_variable = "mon texte" + "autre chose" ! if setted %}
      {% set ma_variable = une_autre_variable + une_autre ! if setted %}
      {% set ma_variable = une_autre_variable + "du texte" ! if setted %}
      {% set ma_variable = "du texte" + une_autre_variable ! if setted %}
      ...
    ```
    _Une variable "vide" (texte vide) est bien une variable existante._
  - Définition d'une variable simple (avec ou sans concaténation), _seulement si elle n'existe pas_
    ```
      {% set ma_variable = "mon texte" ! if unset %}
      {% set ma_variable = une_autre_variable ! if unset %}
      {% set ma_variable = "mon texte" + "autre chose" ! if unset %}
      {% set ma_variable = une_autre_variable + une_autre ! if unset %}
      {% set ma_variable = une_autre_variable + "du texte" ! if unset %}
      {% set ma_variable = "du texte" + une_autre_variable ! if unset %}
      ...
    ```
    _Une variable "vide" (texte vide) est bien une variable existante._

#### Déclaration d'appel d'un bloc (`call`)

__Grammaire locale :__ 
  `{% call [text or symbol] %}`

__Notes :__
  - Un bloc doit toujours exister au moment de son appel. S'il n'est pas défini, une erreur sera générée.
  - Il peut être redéfini autant de fois que souhaité.
  - Un bloc passe d'un traitement à l'autre (il peut donc être défini à un moment et être utilisé plus tard, pour un autre traitement).

__Exemples :__
  - Appel d'un contenu simple :
    ```
    {% block "mon_bloc" %}Ici un contenu{% endblock %}
    {% call "mon_bloc" %}
    ```
  - Appel d'un contenu avec des délimiteurs dans le contenu :
    ```
    {% set ma_variable = "bonjour" %}
    {% block "mon_bloc" %}{{ ma_variable }} le monde !{% endblock %}
    {% call "mon_bloc" %}
    ```
    _Tous les délimiteurs valides (commentaires, expressions, déclarations) sont autorisés._

#### Déclaration de recherche dans un dossier (`find`)

__Grammaire locale :__
  `{% find ['all' or 'files' or 'directories'] in [text or symbol] to [text or symbol] (! [text or symbol]) %}`

__Notes :__
  - Il n'y a pas de protection spécifique sur les chemins lus (c'est-à-la charge du processus parent de créer la prison nécessaire). Les liens symboliques sont résolus.
  - Le chemin doit exister et ne pas être vide (par défaut, la recherche se fait dans le répertoire courant où le processus a été appelé). Si le chemin indiqué est un fichier, il est simplement retourné.
  - Le gabarit de recherche peut contenir `*` pour simuler la présence de 0 à n caractères.
  - Il n'y a pas de récurcivité dans le dossier parcouru.
  - La variable contient les résultats avec le caractère `\n` entre chaque item.
  - Après le séparateur des options `!`, il est possible d'indiquer un texte ou une variable qui soit le charactère de regroupement. 

__Exemples :__
  - Recherche des dossiers :
    ```
    {% find directories in "./articles" to articles %}
    ```
  - Recherche de fichiers selon un gabarit de nom :
    ```
    {% find files in "*.md" to articles %}
    ```
  - Recherche de tous les éléments :
    ```
    {% find all in "/var/log" to journaux %}
    ```
  - Recherche de tous les éléments et jointure avec ";" :
    ```
    {% find all in "." to csv_line ! ";" %}
    ```

#### Déclaration d'inclusion de contenu (`include`)

__Grammaire locale :__
  `{% include [text or symbol] %}`

__Notes :__
  - Il n'y a pas de protection spécifique sur les chemins lus (c'est-à-la charge du processus parent de créer la prison nécessaire). Les liens symboliques sont résolus.
  - Le chemin doit exister et ne pas être vide (par défaut, la recherche se fait dans le répertoire courant où le processus a été appelé). 
  - Le chemin indiqué (par une variable ou directement un texte), doit être un fichier. 
  - Le contenu récupéré est ajouté et n'est pas directement traité (il le sera au passage suivant si `-r` est fourni).
  - Le contenu doit loger en intégralité dans la mémoire ! _Attention aux fichiers volumineux si votre système est déjà en situation de stress._

__Exemples :__
  - Inclusion d'un fichier à partir d'une valeur textuelle :
    ```
    {% include "mon_article.md" %}
    ```
  - Inclusion d'un fichier à partir d'une variable :
    ```
    {% set mon_chemin = "mon_article.md" %}
    {% include mon_chemin %}
    ```

#### Déclaration d'exécution (`execute`)

__Grammaire locale :__
  `{% execute [symbole] = [symbol-1"."symbol-2]"(" [text or symbol] ([text or symbol] ...) ")" (| [symbol"."symbol]"(" [text or symbol] ([text or symbol] ...) ")")%}` - où `symbol-1` est le nom de l'extension, et `symbol-2` le nom de la fonction à appeler

__Notes :__
  - Par défaut la déclaration d'exécution __n'est pas active__ avec l'installateur par défaut de Moustache (`install.sh`), pour des raisons de sécurité et de sûreté. Vous devez rajouter `--features "engine-extensions"` à la commande de compilation pour en bénéficier. 
  - Il s'agit là de pratiques très avancées de Moustache. Dans le cas où vous n'utilisez pas une version nominale de Moustache (par exemple une version modifiée ou étendue) ___N'EXECUTEZ PAS DE CONTENUS DONT VOUS N'ETES PAS SÛR DU CONTENU ET DES EFFETS ATTENDUS___. En effet dans certains cas les extensions peuvent utiliser des commandes du systèmes et permettre à un attaquant, d'agir au travers de la génération de texte. 
  - Par défaut, le code depuis le dépôt principal de Moustache (git.nothus.fr) tenter d'éviter toute extension "ouvertement" dangereuse pour le système. Vous pouvez donc utiliser le code avec une (relative) confiance. 
  - Pour les versions compilés avec des extensions où l'on souhaite en interdire l'usage lors de l'exécution, un argument est disponible : `--no-extensions`.

__Documentations disponibles via le code compilé de Moustache :__
  `moustache --help-extensions`

#### Déclaration de conditionalité (`if`)

__Grammaire locale :__
  `{% if [symbol or text] [ '==' | '!=' ] [symbol or text] ( [ '&' | '|' ] ... ) %}`

__Notes :__
  - Cette déclaration n'est pas imbriquable dans elle-même (pas de `if` directement dans un autre).

__Exemples :__
  - Condition simple :
    ```
    {% if mavar == "1" %}

    {% endif %}
    ```
  - Condition complexe : 
    ```
    {% if mavar == "1" && (oui_ou_non == "oui" || oui_ou_non == "non") %}

    {% endif %}
    ```
  - La logique n'est pas contrôlé, ceci ne sera jamais une condition retournant 'vrai' :
    ```
    {% if mavar == "1" && mavar == "2" %}

    {% endif %}
    ```








## L'auteur 

[Julien Garderon](julien.garderon@gmail.com) - avril 2024
https://mastodon.nothus.fr/@nothus

_Cet applicatif est sous licence GNU GENERAL PUBLIC LICENSE (voir [LICENCE](./LICENCE)) et n'offre aucune garantie quant à son usage. Respectez les droits d'auteur et gardez votre code ouvert si vous ré-utilisez tout ou en partie du code de cette application !_

## Elements techniques divers 

_Ces éléments sont pour les utilisateurs avertis et les développeurs. Si vous ne comprenez pas ce que vous lisez, n'en faites rien ! Vous pourriez possiblement endommager votre système ou simplement rendre l'applicatif inopérant._

### Support d'encodage 

ASCII et UTF8 sont pleinement supportés. UTF16 ainsi que les encodages Windows doivent pouvoir fonctionner - sans garantie aucune. 

### Réduction de la taille 

Voir l'article : https://github.com/johnthagen/min-sized-rust?tab=readme-ov-file

### Lancer l'installation en mode débogue

```bash
DEBUG=1 ./install.sh
```

### Lancer une compilation et la chaîne de tests en mode débogue 

```bash
cargo build --release && DEBUG=1 ./tests/tests.sh "target/release/moustachet" "./tests/*.test"
```

### Lancer Valgrind pour la recherche de bogues 

```bash
valgrind -s --track-origins=yes --leak-check=full target/release/moustache -v "...=..." --output "..." --input "..." -r
```
