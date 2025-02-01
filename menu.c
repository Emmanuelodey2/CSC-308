#include <stdio.h>

void display_menu()
{
    char choice;
    int cart = 0;

    do
    {
        char option;
        int quantity;
        printf("\nMenu:\n");
        printf("P) Poundo yam/Edikanko soup   - N3200\n");
        printf("F) Fried rice/Chicken         - N3000\n");
        printf("A) Amala/Ewedu soup           - N2500\n");
        printf("E) Eba/Egusi soup             - N2800\n");
        printf("W) White rice/Stew            - N2000\n\n");

        printf("What would you like to order? (Enter the corresponding letter): ");

        if (scanf("%c", &option) == 1)
        {
            printf("Enter the quantity: ");
            if (scanf("%d", &quantity) == 1)
            {
                switch (option)
                {
                case 'P':
                case 'p':
                    printf("You selected Poundo yam/Edikanko soup. Price: N3200 x %d = N%d.\n", quantity, 3200 * quantity);
                    cart += 3200 * quantity;
                    break;
                case 'F':
                case 'f':
                    printf("You selected Fried rice/Chicken. Price: N3000 x %d = N%d.\n", quantity, 3000 * quantity);
                    cart += 3000 * quantity;
                    break;
                case 'A':
                case 'a':
                    printf("You selected Amala/Ewedu soup. Price: N2500 x %d = N%d.\n", quantity, 2500 * quantity);
                    cart += 2500 * quantity;
                    break;
                case 'E':
                case 'e':
                    printf("You selected Eba/Egusi soup. Price: N2800 x %d = N%d.\n", quantity, 2800 * quantity);
                    cart += 2800 * quantity;
                    break;
                case 'W':
                case 'w':
                    printf("You selected White rice/Stew. Price: N2000 x %d = N%d.\n", quantity, 2000 * quantity);
                    cart += 2000 * quantity;
                    break;
                default:
                    printf("Invalid option. Please select a valid menu item.\n");
                    break;
                }

                printf("Would you like to order again? (y/n): ");
                scanf(" %c", &choice);
            }
        }
        else
        {
            printf("Invalid Input, try again");
            choice ='y';
            break;
        }

    } while (choice == 'y' || choice == 'Y');

    printf("Your total order cost is: N%d\n", cart);
}

int main()
{
    display_menu();
    return 0;
}
