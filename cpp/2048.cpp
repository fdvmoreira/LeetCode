/**
 * @file 2048.cpp
 * @author your name (you@domain.com)
 * @brief 
 * @version 0.1
 * @date 2022-06-26
 * 
 * @copyright Copyright (c) 2022
 * 
 */



//**** PSEUDOCODE
// READ input from STDIN
// SAVE the input into 4x4 matrix
// GET the movement direction [0 = LETF,1 = TOP,2 = RIGHT,3 = DOWN]
// MOVE rows or columns in the direction entered previouly, WHILE any cell has room to move
//      IF a cell ends next to a similar value cell
//          ADD them to together
//          SET lalter cell's location to ZERO/EMPTY
//      END IF
// END WHILE