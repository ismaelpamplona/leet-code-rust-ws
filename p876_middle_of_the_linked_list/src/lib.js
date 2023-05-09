class LinkedList {
    constructor(val) {
        this.val = val;
        this.next = null;
    }
}

function from_vec_to_linked_list(vec, n = 0) {
    let ll = null;
    if (vec.length === 0) {
        // base case
        return null;
    } else if (n < vec.length) {
        // recursion
        ll = new LinkedList(vec[n]);
        ll.next = from_vec_to_linked_list(vec, n + 1);
    }
    return ll;
}

function from_linked_list_to_vec(linked_list) {
    let vec = [];
    if (linked_list === null) return vec;

    let node = linked_list;
    let bool = true;
    while (bool) {
        vec = [...vec, node.val];
        if (node.next !== null) {
            node = node.next;
        } else {
            bool = false;
        }
    }
    return vec;
}

function middle_node(linked_list) {
    if (linked_list == null) return null;
    let q = [];
    let node = linked_list;
    let bool = true;
    while (bool) {
        q = [...q, node];
        if (node.next !== null) {
            node = node.next;
        } else {
            bool = false;
        }
    }
    const i = q.length % 2 === 0 ? q.length / 2 : (q.length - 1) / 2;
    return q[i];
}

function middle_node_2(linked_list) {
    let mid = linked_list;
    let end = linked_list;
    while (end !== null && end.next !== null) {
        mid = mid.next;
        end = end.next.next;
    }
    return mid;
}

const llVec = [1, 2, 3, 4, 5, 6, 7, 8];
// const llVec = [1];

const ll = from_vec_to_linked_list(llVec);
const middle = middle_node(ll);
const middle2 = middle_node_2(ll);
console.log(middle)
console.log(middle2)

const vecLinkedList = from_linked_list_to_vec(ll);
  // console.log(vecLinkedList);
