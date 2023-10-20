# Check if the values in a linkedlist are polindrome
# O(n) time and O(1) space complexity

class LinkedList:
    def __init__(self, value):
        """Initialize a linked list node with a given value."""
        self.value = value
        self.next = None


def linkedListPalindrome(head):
    """Check if a linked list is a palindrome."""
    if not head or not head.next:
        return True

    num_of_nodes = 0
    curr = head

    # Calculate the number of nodes in the linked list
    while curr:
        num_of_nodes += 1
        curr = curr.next

    middle = num_of_nodes // 2

    # Find the head of the second half
    snd_half_head = head
    for _ in range(middle):
        snd_half_head = snd_half_head.next

    # Reverse the second half of the linked list
    snd_half_head = reverse_linked_list(snd_half_head)

    first_half, second_half = head, snd_half_head

    # Compare the first and second halves for palindrome check
    while second_half:
        if first_half.value != second_half.value:
            return False
        first_half = first_half.next
        second_half = second_half.next

    return True


def reverse_linked_list(snd_head):
    """Reverse a linked list and return the new head."""
    prev, curr = None, snd_head
    while curr:
        tmp = curr.next
        curr.next = prev
        prev = curr
        curr = tmp
    return prev
