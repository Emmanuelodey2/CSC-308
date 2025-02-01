#include <stdio.h>

int main() {
    char ch;

    // Take input from the user
    printf("Enter a character between A and J: ");
    scanf("%c", &ch);

    // Check if the character is between A and J
    if ((ch >= 'A' && ch <= 'J') || (ch >= 'a' && ch <= 'j')) {
        printf("The next 6 characters are: ");
        
        // Print the next 6 characters in sequence
        for (int i = 1; i <= 6; i++) {
            printf("%c ", ch + i);
        }
        printf("\n");
    } else {
        printf("Invalid input. Please enter a character between A and J.\n");
    }

    return 0;
}
