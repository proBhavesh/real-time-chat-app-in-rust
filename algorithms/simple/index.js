let arr = [2, 3, 4, 5];

//factorial algorithm using recurssion
const factorial = (arr) => {
  let b = [];
  // for (i = 0; i <arr.length; i++) {
    if (arr[i] == 0 || arr[i] == 1) {
      console.log("this is one", arr[i]);
      return 1;
    } else {
      console.log("this is non-one", arr[i]);
      return b.push(arr[i] * factorial(arr[i] - 1))
    }
    return b;
  // }
};


console.log(factorial(arr));
