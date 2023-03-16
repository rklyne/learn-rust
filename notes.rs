/** example from discord
*

   #[derive(Copy, Clone, Debug, Eq, PartialEq)]
   pub enum Direction {
     North = 0,
     South = 1,
     East = 2,
     West = 3,
   }

   impl Direction {
     pub fn turn_left(self) -> Self { ... }
   }

   #[test]
   fn turning_left_four_times() {
     assert_eq!(Direction::North, Direction::North.turn_left().turn_left().turn_left().turn_left());
   }
   ￼

*/


