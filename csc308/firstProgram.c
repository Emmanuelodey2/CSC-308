#include <stdio.h>

int main() {
    int age;

    // Prompt the user to enter their age
    printf("Please enter your age: ");

    // Check if the input is valid
    if (scanf("%d", &age) !=1 ) {
        // Error handling for invalid input
        printf("Invalid input! Please enter a valid age.\n");
        return 1; // Exit the program with an error code
    }

    // Check if the age is greater than 18
    if (age > 18) {
        printf("You can vote.\n");
    } else {
        printf("You cannot vote.\n");
    }

    return 0;
}
