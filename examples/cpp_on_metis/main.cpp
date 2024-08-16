#include <iostream>

int main () {
    // Say hello to the user
    std::cout << "Hello, Metis!" << std::endl;

    // Initialize our counter variables
    unsigned long long int counter = 0;
    unsigned long long int number_of_divisible_by_two = 0;
    unsigned long long int number_of_divisible_by_three = 0;
    unsigned long long int number_of_divisible_by_five = 0;

    // First, iterate through a 3D grid to get to our block
    for ( int grid_z = 0; grid_z < 1000; grid_z++ ) {
        for ( int grid_y = 0; grid_y < 100; grid_y++ ) {
            for ( int grid_x = 0; grid_x < 100; grid_x++ ) {

                // Second, iterate through the 3D block
                for ( int block_z = 0; block_z < 10; block_z++ ) {
                    for ( int block_y = 0; block_y < 10; block_y++ ) {
                        for ( int block_x = 0; block_x < 10; block_x++ ) {
                            counter += 1;

                            if ( counter % 2 == 0 )
                                number_of_divisible_by_two += 1;
                            if ( counter % 3 == 0 )
                                number_of_divisible_by_three += 1;
                            if ( counter % 5 == 0 )
                                number_of_divisible_by_five += 1;
                        }
                    }
                }

            }
        }
    }

    // Provide our results to the user
    std::cout << std::endl
              << "- Numbers divisible by two: "       << number_of_divisible_by_two       << std::endl
              << "- Numbers divisible by three: "     << number_of_divisible_by_three     << std::endl
              << "- Numbers divisible by five: "      << number_of_divisible_by_five      << std::endl;

    return 0;
}