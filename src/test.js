const flatten = (arr) => {
    let res = [];

    for (let i = 0; i < arr.length; i++ ) {
        if (!Array.isArray (arr[i])) {
            res.push(arr[i]);
            continue;
        }
        let stack = arr[i];
        let localRes = [];
        while (stack.length) {
            stackElem = stack.pop();
            if (Array.isArray(stackElem)) {
                stack.push(...stackElem);
            } else {
                localRes.push(stackElem);
            }
        }
        res.push(...localRes.reverse());
    }
    return res;
}


console.log(flatten([[1,2], [[3], 4]]));
console.log(flatten([[[1, [2], 3], 4], [[[[5]]]]]));