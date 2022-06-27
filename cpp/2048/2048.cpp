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

#include <iostream>

/**
 * @brief name the Directions
 *
 */
enum class Direction
{
    LEFT = 0,
    TOP = 1,
    RIGHT = 2,
    BOTTOM = 3
};

/**
 * @brief read the values from the STDIN and save it into matrix
 *
 * @param matrix
 */
void loadMatrix(int (&matrix)[4][4])
{
    for (size_t i = 0; i < 4; i++)
    {
        // std::cin >> matrix[i][0] matrix[i][1] matrix[i][2] matrix[i][3];
    }
}

int main(int argc, char **argv)
{
    int matrix[4][4] = {{0, 0, 0, 0},
                        {0, 0, 0, 0},
                        {0, 0, 0, 0},
                        {0, 0, 0, 0}};

    // Direction &movementDirection;
    Direction movementDirection;

    std::cout << " Enter values: ";
    int val[4];
    int a,b;
    std::cin >> a,b;
    std::cout << a<<b << std::endl;
    return 0;
}