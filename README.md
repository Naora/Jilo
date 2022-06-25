# Jilo

## But du projet

dans un monde ou l'on veux faire du dynamique a tout prix coté serveur je souhaiterais revenir un peu en arriere pour faire quelque choses de plus rapide par nature.

Ma proposition est de creer un CMS qui génère du contenue static et qui est rendu dynamique via des API.

## Qui sont les acteurs qui sont capable d'interagir avec le system ?

- Utilisateur : Une personne qui peu modifier et publié le site
- Developpeur : Une personne capable de modifier le comportement et l'aspect du site
- Admin : Une personne etant capable d'ajouter/modifier des droits sur le site

## CMS

L'idee d'un CMS est de données la posibilité a des utilisateurs sans connaissance technique de modifier et publié un site web.

## Module

L'idee de module est de données la possibilité de faire du plug and play.
Un developpeur ajoute des fonctionnalités et les ajoutes aux site.
L'utilisateur lui peu ajouter ce module a des pages.

## Back (Rust)

Rust est un langage performant et qui laisse peu de marge d'erreur. Donc idéal pour un serveur web. Cela permettra de l'installer même sur un raspberry pi.

## Front (Lit)

On va utilisez du lit.dev pour la partie PWA. A voir si on fait a terme une SPA.
Avant j'ai essayer ELM. Très sympas a utiliser mais trop complexe aux globale.

## Orchestrateur de creation (Make)

Le plus basique c'est d'utiliser make qui permet d'automatiser tout la gestion des différents projets entre eux.
