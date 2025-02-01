#include <stdio.h>

int main() {
    int experience, age;
    float salary;

    // Take input for experience and age with error handling
    printf("Enter years of experience: ");
    if (scanf("%d", &experience) != 1 || experience < 0) {
        printf("Invalid input for experience. Please enter a non-negative integer.\n");
        return 1;  // Exit the program on invalid input
    }

    printf("Enter age: ");
    if (scanf("%d", &age) != 1 || age < 0) {
        printf("Invalid input for age. Please enter a non-negative integer.\n");
        return 1;  // Exit the program on invalid input
    }

    // Determine salary based on experience and age
    if (experience >= 5) {  // Experienced person (assumption: >= 5 years of experience)
        if (age >= 40) {
            salary = 560000;  // Salary for experienced and age >= 40
        } else if (age >= 30) {
            salary = 480000;  // Salary for experienced and age between 30 and 39
        } else if (age >= 28) {
            salary = 300000;  // Salary for experienced and age between 28 and 29
        } else {
            printf("Invalid age for experienced person.\n");
            return 1;
        }
    } else {  // Inexperienced person
        salary = 100000;  // Salary for inexperienced person
    }

    // Print the calculated salary
    printf("The salary is: N%.2f\n", salary);

    return 0;
}
