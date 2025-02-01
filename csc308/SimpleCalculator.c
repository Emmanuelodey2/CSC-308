#include <stdio.h>
void calculateSum() {
float num1,num2;
  printf("Enter Your first number:");
  scanf("%f", &num1);
  printf("Enter Your second number:");
  scanf("%f", &num2);
 printf("Enter Your second number:%.2f, %.2f", num1, num2);
  float sum = num1 + num2;
  printf("\nThe sum of x + y is: %.2f", sum);
}
void calculateMinus() {
float num1,num2;
  printf("Enter Your first number:");
  scanf("%f", &num1);
  printf("Enter Your second number:");
  scanf("%f", &num2);
 printf("Enter Your second number:%.2f, %.2f", num1, num2);
  float sum = num1 - num2;
  printf("\nThe sum of x + y is: %.2f", sum);
}
void calculateAverage() {
float num1,num2;
  printf("Enter Your first number:");
  scanf("%f", &num1);
  printf("Enter Your second number:");
  scanf("%f", &num2);
 printf("Enter Your second number:%.2f, %.2f", num1, num2);
  float sum = (float) num1 / num2;
  printf("T\nhe sum of x + y is: %.2f", sum);
}
void calculateProduct() {
float num1,num2;
  printf("Enter Your first number:");
  scanf("%f", &num1);
  printf("Enter Your second number:");
  scanf("%f", &num2);
 printf("Enter Your second number:%.2f, %.2f", num1, num2);
  float sum = (float) num1 * num2;
  printf("\nThe sum of x + y is: %.2f", sum);
}
int main() {
    int input;
    printf("Welcome to my calculator\n Choose 1.Addition 2.Divition 3.Multiplication 4.Subtraction");
    scanf("%d", &input);
    switch (input){
        case 1 :{
            calculateSum();
            break;
        }
        case 2 :{
            calculateAverage();
            break;
        }
        case 3 :{
            calculateProduct();
            break;
        }
        case 4 :{
            calculateMinus();
            break;
        }
    }


  
    return 0;
}
