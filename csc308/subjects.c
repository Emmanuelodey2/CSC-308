#include <stdio.h>

int main() {
    // Declare variables for the three subjects
    float csc201, csc205, sta205;
    float total, average, percentage;

    // Take input for each subject with error handling
    printf("Enter marks for CSC 201: ");
    if (scanf("%f", &csc201) != 1 || csc201 < 0 || csc201 > 100) {
        printf("Invalid input for CSC 201. Please enter a number between 0 and 100.\n");
        return 1;  // Exit program on invalid input
    }

    printf("Enter marks for CSC 205: ");
    if (scanf("%f", &csc205) != 1 || csc205 < 0 || csc205 > 100) {
        printf("Invalid input for CSC 205. Please enter a number between 0 and 100.\n");
        return 1;  // Exit program on invalid input
    }

    printf("Enter marks for STA 205: ");
    if (scanf("%f", &sta205) != 1 || sta205 < 0 || sta205 > 100) {
        printf("Invalid input for STA 205. Please enter a number between 0 and 100.\n");
        return 1;  // Exit program on invalid input
    }

    // Calculate total, average, and percentage
    total = csc201 + csc205 + sta205;
    average = total / 3;
    percentage = (total / 300) * 100;  // Total marks for three subjects is 300

    // Display the results
    printf("\nTotal Marks: %.2f\n", total);
    printf("Average Marks: %.2f\n", average);
    printf("Percentage: %.2f%%\n", percentage);

    return 0;
}
