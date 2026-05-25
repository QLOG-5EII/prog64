# Prog64

## Afficher un programme

Écrire la fonction `print_program` du fichier `program.rs`. 
Cette fonction affiche un programme, ligne par ligne. Par exemple, 
voici à quoi ressemble un programme : 

```
===============
r0 = abs r0 r0
r0 = ld r0 i3
r1 = add r1 r0
r1 = add r1 r1
r1 = add r1 r1
r0 = add r0 i1
===============
```

`r0`...`r3` sont les registres. 
`i0`...`i3` sont les inputs. 
`add`, `ld` et `abs` sont les instructions (voir `INSTRUCTIONS_NAME` dans `instructions.rs`). 

> En plus de la fonction `println!`, il peut être intéressant de regarder la fonction `print!` qui fait la même chose, mais sans passage à la ligne. Également, la fonction `format!` qui renvoie un string plutôt que de l'afficher.

> Il est possible de s'inspirer de la fonction `eval_prog` de `program.rs` pour la lecture d'un programme.

## Implémentation de test unitaire 

Écrire un test unitaire pour la fonction `eval_prog` de `program.rs`.
Cette fonction évalue un programme à partir d'inputs. L'objectif ici est de créer un programme vous-même 
et de prédire la sortie à partir d'inputs que vous choisissez.

Voir : [Unit tests](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)

## Implémentation de tests d'intégration

Écrire un test d'intégration pour tester un entraînement. À partir d'un seed, l’entraînement devrait 
être déterministe. À vous de déterminer comment tester que le résultat ne change pas.

Voir : [Integration tests](https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html)
