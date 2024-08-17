#include <iostream>

/// A kernel function designed to calculate the number of
///  numbers divisible by two, three, and five
///
/// # Arguments
/// * `d_number_of_divisible_by_two` - The number of numbers divisible by two
/// * `d_number_of_divisible_by_three` - The number of numbers divisible by three
/// * `d_number_of_divisible_by_five` - The number of numbers divisible by five
__global__ void calculate(
    unsigned long long int * d_number_of_divisible_by_two,
    unsigned long long int * d_number_of_divisible_by_three,
    unsigned long long int * d_number_of_divisible_by_five
) {
    int grid_x = blockIdx.x;
    int grid_y = blockIdx.y;
    int grid_z = blockIdx.z;

    int block_x = threadIdx.x;
    int block_y = threadIdx.y;
    int block_z = threadIdx.z;

    unsigned long long local_counter = 
        (grid_z * 100 * 100 * 10 * 10 * 10) + 
        (grid_y * 100 * 10 * 10) + 
        (grid_x * 10 * 10) +
        (block_z * 10 * 10) +
        (block_y * 10) +
        block_x + 1;

    unsigned long one = 1;

    if (local_counter % 2 == 0) {
        atomicAdd(d_number_of_divisible_by_two, one);
    }
    if (local_counter % 3 == 0) {
        atomicAdd(d_number_of_divisible_by_three, one);
    }
    if (local_counter % 5 == 0) {
        atomicAdd(d_number_of_divisible_by_five, one);
    }
}

int main() {
    // Say hello to the user
    std::cout << "Hello, Metis!" << std::endl;

    // Host variables
    unsigned long long int h_number_of_divisible_by_two   = 0;
    unsigned long long int h_number_of_divisible_by_three = 0;
    unsigned long long int h_number_of_divisible_by_five  = 0;

    // Device variables
    unsigned long long int * d_number_of_divisible_by_two;
    unsigned long long int * d_number_of_divisible_by_three;
    unsigned long long int * d_number_of_divisible_by_five;

    // Allocate memory on the device with the correct sizing
    cudaMalloc( &d_number_of_divisible_by_two,   sizeof(unsigned long long int) );
    cudaMalloc( &d_number_of_divisible_by_three, sizeof(unsigned long long int) );
    cudaMalloc( &d_number_of_divisible_by_five,  sizeof(unsigned long long int) );

    // Copy the memory from the host to the device
    cudaMemcpy( d_number_of_divisible_by_two,   &h_number_of_divisible_by_two,   
        sizeof(unsigned long long int), cudaMemcpyHostToDevice );
    cudaMemcpy( d_number_of_divisible_by_three, &h_number_of_divisible_by_three,
        sizeof(unsigned long long int), cudaMemcpyHostToDevice );
    cudaMemcpy( d_number_of_divisible_by_five,  &h_number_of_divisible_by_five,
        sizeof(unsigned long long int), cudaMemcpyHostToDevice );

    // Define our grid's dimensions
    dim3 gridDim(100, 100, 10);

    // Define each block's dimensions
    dim3 blockDim(10, 10, 10);

    // Run our calculation
    calculate<<<gridDim, blockDim>>>(d_number_of_divisible_by_two, d_number_of_divisible_by_three, d_number_of_divisible_by_five);
    cudaDeviceSynchronize();

    // Copy the memory back to our machine
    cudaMemcpy(&h_number_of_divisible_by_two, d_number_of_divisible_by_two, sizeof(unsigned long long int), cudaMemcpyDeviceToHost);
    cudaMemcpy(&h_number_of_divisible_by_three, d_number_of_divisible_by_three, sizeof(unsigned long long int), cudaMemcpyDeviceToHost);
    cudaMemcpy(&h_number_of_divisible_by_five, d_number_of_divisible_by_five, sizeof(unsigned long long int), cudaMemcpyDeviceToHost);

    // Provide our results to the user
    std::cout << std::endl
              << "- Numbers divisible by two: "       << h_number_of_divisible_by_two       << std::endl
              << "- Numbers divisible by three: "     << h_number_of_divisible_by_three     << std::endl
              << "- Numbers divisible by five: "      << h_number_of_divisible_by_five      << std::endl;

    // Free the memory
    cudaFree(d_number_of_divisible_by_two);
    cudaFree(d_number_of_divisible_by_three);
    cudaFree(d_number_of_divisible_by_five);

    return 0;
}