class TreeNode {
    constructor(val) {
        this.val = val;
        this.left = null;
        this.right = null;
    }
}

function from_tn_to_vec(tn, vec) {
    if (vec.length == 0) {
        vec.push(tn.val)
        from_tn_to_vec(tn, vec)
    }

    else {
        if (tn.left !== null) vec.push(tn.left.val)
        if (tn.right !== null) vec.push(tn.right.val)
        if (tn.left !== null) from_tn_to_vec(tn.left, vec)
        if (tn.right !== null) from_tn_to_vec(tn.right, vec)
    }
    return vec
}

let left_2 = new TreeNode(1);
let right_2 = new TreeNode(3);

let left_7 = new TreeNode(6);
let right_7 = new TreeNode(9);

let left_4 = new TreeNode(2);
left_4.left = left_2;
left_4.right = right_2;

let right_4 = new TreeNode(7);
right_4.left = left_7;
right_4.right = right_7;

// let root = new TreeNode(4);
// root.left = left_4;
// root.right = right_4;

// console.log(from_tn_to_vec(root, []));


// let root_2 = new TreeNode(4);
// root_2.left = null
// root_2.right = null

// console.log(from_tn_to_vec(root_2, []));

// let root_3 = new TreeNode(4);
// root_3.left = new TreeNode(2);
// root_3.right = null

// console.log(from_tn_to_vec(root_3, []));

left_2.left = new TreeNode(100);
right_2.right = new TreeNode(200);


let root_4 = new TreeNode(4);
root_4.left = left_4;
root_4.right = right_4;

console.log(from_tn_to_vec(root_4, []));

function from_vec_to_tn(vec, n) {
    let tn = null;
    if (vec.length === 0) {
        console.log("base case");
        return null;
    } else if (n < vec.length) {
        console.log("recursion");
        tn = new TreeNode(vec[n]);
        tn.left = from_vec_to_tn(vec, 2 * n + 1);
        tn.right = from_vec_to_tn(vec, 2 * n + 2);
    }
    return tn;
}

const tnVec = [4, 2, 7, 1, 3, 6, 9, 10, 11, 12];
// const tnVec = [];

console.log(from_vec_to_tn(tnVec, 0));

function invert_tree(tn) {
    if (tn === null) {
        return null;
    } else {
        let left = tn.left !== null ? tn.left : null;
        let right = tn.right !== null ? tn.right : null;
        tn = new TreeNode(tn.val);
        tn.left = invert_tree(right);
        tn.right = invert_tree(left);
    }
    return tn;
}
