# Reverse Linked List in-place and in linear time
#

class LinkedList:
    def __init__(self, value):
        self.value = value
        self.next = None


def reverseLinkedList(head):
    # Initialize pointers for previous, current, and temporary nodes
    prev = None
    curr = head

    while curr:
        # Store the next node in a temporary variable
        next_node = curr.next
        # Reverse the link to point to the previous node
        curr.next = prev
        # Move prev and curr pointers one step forward
        prev = curr
        curr = next_node

    # The new head of the reversed list is the previous node
    return prev

# O(n) time | O(1) space complexity
