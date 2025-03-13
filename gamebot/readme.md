# Bots en Solo et en Équipe

Ce projet implémente des bots pouvant fonctionner en solo ou en équipe en se connectant à un serveur via TCP.

## Structure du Projet

- `bot.rs` : Définit la structure d'un bot individuel et sa logique de connexion au serveur.
- `team.rs` : Permet de lancer une équipe de bots avec un nombre configurable de participants.
- `main.rs` : Point d'entrée principal pour démarrer les bots.

## Installation et Compilation

Ce projet est écrit en Rust. Assurez-vous d'avoir installé Rust et Cargo.

```
cargo build --release
```

## Utilisation

### Démarrer un bot seul

```
cargo run --release -- --server_address 172.17.0.1 --server_port 6969
```

### Démarrer une équipe de bots

```
cargo run --release --bin team -- --num_bots 3 --server_address 172.17.0.1 --server_port 6969
```

### Options disponibles

| Option | Description |
|--------|-------------|
| `--server_address` | Adresse IP du serveur |
| `--server_port` | Port du serveur |
| `--num_bots` | Nombre de bots à lancer (pour `team.rs`) |

## Fonctionnement du Bot

Le bot fonctionne en se connectant à un serveur TCP et en exécutant une série d'actions prédéfinies basées sur son environnement. Voici les principales fonctionnalités du bot :

### Logique de Connexion

- Lors du démarrage, le bot se connecte à l'adresse IP et au port du serveur définis dans les arguments de la ligne de commande.
- Il s'identifie au serveur en envoyant son nom avec la commande `NAME=<nom_du_bot>`.

### Communication avec le Serveur

Le bot utilise des commandes spécifiques pour interagir avec le serveur. Ces commandes sont envoyées en format texte. Voici quelques exemples de commandes utilisées :
- **MotL=<valeur>** : Définit la puissance du moteur gauche (valeur entre -1.0 et 1.0).
- **MotR=<valeur>** : Définit la puissance du moteur droit (valeur entre -1.0 et 1.0).
- **GunTrav=<valeur>** : Définit la position du canon (valeur entre 0.0 et 1.0).
- **GunTrig=<valeur>** : Déclenche ou arrête le tir (`1.0` pour tirer, `0.0` pour arrêter).
- **MSG=<message>** : Envoie un message de statut au serveur.
- **NLIST** : Récupère la liste des joueurs présents sur le serveur.
- **NBOT=<nom_du_joueur>** : Demande des informations sur un joueur spécifique.

### Comportement en Jeu

Le comportement du bot repose sur plusieurs étapes principales :

#### 1. Détection et Suivi des Ennemis
- Le bot utilise la commande `NLIST` pour récupérer la liste des joueurs connectés.
- Il identifie les ennemis basés sur leur nom (les ennemis n'ont pas de préfixe spécifique comme `TeamAlpha-`).
- Une fois un ennemi détecté, le bot peut obtenir ses informations (orientation, distance) et ajuster ses mouvements pour le traquer.

#### 2. Mouvement et Poursuite
Le bot ajuste ses moteurs en fonction de la position de l'ennemi :
- Si l'ennemi est directement en face (orientation proche de 0.5), le bot avance tout droit.
- Si l'ennemi est à gauche ou à droite, le bot ajuste les puissances des moteurs pour se tourner vers lui.
- En l'absence d'ennemi détecté, le bot exécute des mouvements de recherche, alternant entre avancer en ligne droite et tourner à des angles fixes.

#### 3. Tir Précis
- Le bot vise son canon sur l'ennemi à l'aide de la commande `GunTrav`.
- Il calcule dynamiquement une position optimale pour le canon en intégrant l'orientation de l'ennemi, sa distance et un petit facteur d'anticipation basé sur le mouvement.
- Le bot décide de tirer uniquement si la précision estimée est suffisante (basée sur l'orientation relative à l'ennemi et à sa distance).

#### 4. Évitement des Projectiles
- Le bot utilise la commande `CPROJ` pour détecter les projectiles en approche.
- Si un projectile est détecté à une distance critique (par exemple, moins de 70 unités), le bot exécute des manœuvres d'évitement en modifiant ses moteurs selon des schémas imprévisibles (par exemple, virages serrés, zigzags, reculs rapides).

#### 5. Messages de Statut
Le bot communique régulièrement avec le serveur pour envoyer des messages de statut :
- Par exemple, lorsqu'il reprend contact avec un ennemi, il envoie `MSG=Reprise de contact...`.
- Lors d'une attaque, il envoie des messages comme `MSG=Cible acquise - FEU!`.

### Méthodes Clés

#### `track_and_shoot_enemy()`
Méthode principale du bot, gérant la détection, la poursuite et l'engagement des ennemis. Elle combine les fonctionnalités suivantes :
- Identification des ennemis via `get_closest_enemy_fast()`.
- Ajustement des moteurs via `hunt_enemy()`.
- Vise et tire sur les cibles à portée via `aim_and_shoot()`.

#### `evasive_maneuvers()`
Méthode utilisée pour éviter les projectiles. Elle applique des motifs aléatoires de mouvement pour minimiser la probabilité d'être touché.

#### `search_ennemy()`
Méthode de recherche utilisée lorsque l'ennemi est perdu de vue. Le bot exécute des mouvements automatiques, alternant entre avancer tout droit et tourner selon un angle fixe, jusqu'à détecter une nouvelle cible.

### Tests Unitaires

Le bot est accompagné de tests unitaires pour garantir la qualité de son comportement :
- **Création d'un bot** : Vérifie que le bot est créé avec les bons paramètres de connexion.
- **Calcul de la position optimale du canon** : Valide que le canon est correctement aligné sur l'ennemi.
- **Décision de tir** : Vérifie que le bot tire uniquement lorsque les conditions de précision et de distance sont satisfaites.
- **Poursuite et recherche** : Teste les algorithmes de suivi et de recherche des ennemis.
- **Vérification des ennemis** : S'assure que les ennemis sont reconnus correctement selon les noms.

### Optimisations
1. **Précision dynamique pour le tir** : La logique de tir du bot s'ajuste en fonction de la distance. Plus la cible est éloignée, plus il devient exigeant en termes de précision.
2. **Recherche intelligente** : Lorsqu'aucun ennemi n'est trouvé, le bot alterne automatiquement entre des mouvements d'exploration pour améliorer ses chances de détection.
3. **Manœuvres d'évitement avancées** : Les motifs d'esquive changent régulièrement pour rendre les mouvements du bot plus imprévisibles.


## Dépendances

Ce projet utilise les bibliothèques suivantes :

- `clap` : Gestion des arguments en ligne de commande.
- `rand` : Génération de valeurs aléatoires.
- `std::net` : Gestion des connexions réseau.
- `std::sync` : Synchronisation entre threads.

## Contribution

Les contributions sont les bienvenues ! Vous pouvez soumettre des pull requests ou ouvrir des issues sur le dépôt Git.
