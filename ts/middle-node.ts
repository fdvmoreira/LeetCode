/* Problem
 *
		You're given a Linked List with at least one node. Write a function
		that returns the middle node of the Linked List. If there are two middle
		nodes (i.e. an even length list), your function should return the second
		of these nodes.

 *
 *
 *
 * */

// This is an input class. Do not edit.
export class LinkedList {
	value: number;
	next: LinkedList | null;

	constructor(value: number) {
		this.value = value;
		this.next = null;
	}
}

export function middleNode(linkedList: LinkedList) {
	// Write your code here.
	if (!linkedList) return null;

	const lLength = getLinkedListLength(linkedList);

	if (lLength == 1) return linkedList;

	let mid: number = Math.ceil(lLength / 2);

	if (lLength % 2 == 0) mid += 1;

	let midNode: LinkedList | null = null;
	let curNode: LinkedList | null = linkedList;

	while (mid != 1 && curNode) {
		curNode = curNode.next;
		mid -= 1;
	}

	midNode = curNode;

	return midNode;
}

function getLinkedListLength(linkedlist: LinkedList | null): number {
	let length = 0;
	while (linkedlist) {
		length += 1;
		linkedlist = linkedlist.next;
	}
	return length;
}
