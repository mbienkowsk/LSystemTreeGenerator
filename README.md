[Architecture docs](/docs/architecture.md)

### Running
Clone, `just run`. Check out `just -l` for recipes.

### Controls
* Move using WASD/HJKL
* Space to move up, z to move down
* Mouse to look around
* Press `Esc` to enter menu

## Specification

W ramach projektu należy stworzyć program, który będzie umożliwiał two-
rzenie roślin z wykorzystaniem L-systemów.
W tym celu należy zaimplementować:
1. [x] wsparcie dla podstawowego typu plików wejściowych z modelem 3D: obj
   - ewentualna baza dla rośliny i podłoże
2. [x] cieniowanie Phonga
3. interpretację właściwości materiału: diffuse
4. [x] kamerę perspektywiczną - możliwość poruszania się po scenie oraz obrotu
5. możliwość generacji roślin:
   - możliwość wyboru 3 różnych predefiniowanych roślin
   - możliwość podania swoich zasad generacji
   - rozsiewanie modeli w zadanym obszarze - należy pamiętać, o transformacjach, żeby roślinność nie wyglądała zbyt sztucznie
6. zmienny kolor rośliny w zależności od jej wysokości
