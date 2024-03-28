# CIFRA DE MUBASHIR

🎯 OBJETIVO
Crie uma função que recebe uma string contendo uma mensagem a ser codificada e uma matriz 2D de letras chave fornecida. A função deve substituir cada letra da mensagem pela letra correspondente na matriz chave. As letras não-alfabéticas devem permanecer em suas posições originais.

👨‍⚖️ REGRAS
A matriz chave é um array bidimensional de pares de letras. Cada par consiste em uma letra minúscula e uma letra maiúscula.
A função deve substituir cada letra da mensagem pela letra correspondente na matriz chave. Por exemplo, se a matriz chave é ['m', 'c'], ['u', 'e'], ['b', 'g'], ['a', 'k'], então a letra 'm' deve ser substituída por 'c', a letra 'u' deve ser substituída por 'e', e assim por diante.
As letras não-alfabéticas (como espaços e pontuação) devem permanecer em suas posições originais.
A função deve retornar a mensagem codificada.

💡 EXEMPLO
Dada a matriz chave:

```
key = [['m', 'c'], ['u', 'e'], ['b', 'g'], ['a', 'k'],
   ['s', 'v'], ['h', 'x'], ['i', 'z'], ['r', 'y'],
   ['p', 'w'], ['l', 'n'], ['o', 'j'], ['t', 'f'], ['q', 'd']]
```


E a mensagem:
message = "challenges da impulso !"


A função deve retornar:

```
"mxknnulbuv qk zcwenvj !"
```

Porém, as letras não-alfabéticas e o espaço entre a mensagem e o ponto de exclamação permanecem em suas posições originais.
