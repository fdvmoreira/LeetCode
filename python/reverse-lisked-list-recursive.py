class LinkedList:
    def __init__(self, value):
        self.value = value
        self.next = None


def helper(prev, curr):
    # Base case: If the current node is None, return the previous node as the new head
    if not curr:
        return prev
    # Store the next node to avoid losing the reference
    next_node = curr.next
    # Reverse the link by pointing the current node's 'next' to the previous node
    curr.next = prev
    # Move the previous and current nodes one step forward
    prev_node = curr
    current_node = next_node
    # Recursively reverse the rest of the linked list
    reversed_head = helper(prev_node, current_node)
    return reversed_head


def reverseLinkedList(head):
    # Initialize the previous node to None and the current node to the head
    prev_node = None
    current_node = head
    # Start the recursion with the initial values
    reversed_head = helper(prev_node, current_node)
    return reversed_head  # Return the new head of the reversed linked list

# O(n) time | O(n) space complexity - where n is the numer of nodes in the list (iterative approach has O(1) space complexity -- refer to **reverse-linked-list.py**)
