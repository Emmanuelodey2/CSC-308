#include <stdio.h>

int main() {
   int input;
   
    printf("Enter a 1.character\n 2.integer\n 3.float: ");
    int res = scanf("%d", &input);  // Use & to get the address of input
    switch (input) {
        case 1: {
         char charValue;
         printf("Enter a character: ");
         scanf(" %c", &charValue);  // Space before %c to consume any trailing whitespace
         printf("You entered a character: %c\n", charValue);
         for (int i = 1; i <= 6; i++) {
            printf("%c\n ", charValue + i);
        }
         printf("Character ASCII value: %d\n", charValue);
         break;
        }
        case 2: {
         int intValue;
         printf("Enter an integer: ");
         scanf("%d", &intValue);  // Pass the address of intValue
         printf("You entered an integer: %d\n", intValue);
         for (int i = 1; i <= 6; i++) {
            printf("%d\n ", intValue + i);
        }
         printf("Integer squared: %d\n", intValue * intValue);
         break;
        }
        case 3: {
         float floatValue;
         printf("Enter a float value: ");
         scanf("%f", &floatValue);  // Pass the address of floatValue
         printf("You entered a float: %.2f\n", floatValue);
         for (int i = 1; i <= 6; i++) {
            printf("%.2f\n ", floatValue + i);
        }
         printf("Float doubled: %.2f\n", floatValue * 2);
         break;
        }
        default:
            printf("Invalid input.\n");
    }

    return 0;
}
