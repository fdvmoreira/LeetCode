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
//TODO > READ input from STDIN
//TODO > SAVE the input into 4x4 matrix
//TODO > GET the movement direction [0 = LEFT,1 = TOP,2 = RIGHT,3 = DOWN]
//TODO > MOVE rows or columns in the direction entered previously, WHILE any cell has room to move
//TODO >      IF a cell ends next to a similar value cell
//TODO >          ADD them to together
//TODO >          SET latter cell's location to ZERO/EMPTY
//TODO >      END IF
//TODO > END WHILE

#include <iostream>
#include <stdexcept>

/**
 * @brief name the Directions
 *
 */
enum class Direction {
    DEFAULT [[maybe_unused]] = -1,
    LEFT [[maybe_unused]] = 0,
    TOP [[maybe_unused]] = 1,
    RIGHT [[maybe_unused]] = 2,
    BOTTOM [[maybe_unused]] = 3,
};

/**
 * Custom Exception To handle Directions
 */
class InvalidDirectionException : private std::exception {
private:
    const char *message;
public:
    explicit InvalidDirectionException(const char *msg) : message(msg) {};

    [[nodiscard]] const char *what() const noexcept override {
        return this->message;
    }
};

/**
 * @brief read the values from the STDIN and save it into matrix
 * @param matrix
 */
void loadMatrix(int (&matrix)[4][4]) {
    for (size_t i = 0; i < 4; i++) {
        std::cout << "rows " << i + 1 << ": ";
        std::cin >> matrix[i][0] >> matrix[i][1] >> matrix[i][2] >> matrix[i][3];
    }
}

/**
 * @brief print the content of the matrix
 * @param matrix
 */
void printMatrix(int (&matrix)[4][4]) {
    for (const auto &row: matrix) {
        std::cout << std::endl;
        for (const int &j: row) {
            std::cout << j << " ";
        }
    }
}

/**
 * @brief get the right direction from the user's input
 * @return Direction of the movement
 */
Direction getMovementDirectionFromUser(const int &_direction) {
    Direction direction;
    switch (_direction) {
        case 0:
            direction = Direction::LEFT;
            break;
        case 1:
            direction = Direction::TOP;
            break;
        case 2:
            direction = Direction::RIGHT;
            break;
        case 3:
            direction = Direction::BOTTOM;
            break;
        default:
            throw InvalidDirectionException("The direction provided is invalid!");
    }
    return direction;
}

int main([[maybe_unused]] int argc, [[maybe_unused]] char **argv) {
    int matrix[][4] = {{0, 0, 0, 0},
                       {0, 0, 0, 0},
                       {0, 0, 0, 0},
                       {0, 0, 0, 0}};

    // Direction &movementDirection;
    [[maybe_unused]] Direction movementDirection;

    std::cout << " Enter values below!\n";
    loadMatrix(matrix);
    printMatrix(matrix);
    int direction;
    movementDirection = getMovementDirectionFromUser(direction);


    return 0;
}


