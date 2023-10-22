class LinkedList:
    def __init__(self, value):
        self.value = value
        self.next = None


def get_linked_list_length(linked_list):
    """Return the length of the linked list."""
    length = 0
    curr = linked_list
    while curr:
        length += 1
        curr = curr.next
    return length


def reverse_linked_list(head):
    """Reverse linked list."""
    prev, curr = None, head

    while curr:
        tmp = curr.next
        curr.next = prev
        prev = curr
        curr = tmp

    return prev


def merge_linked_lists(first_head, second_head):
    """Merge two linked lists."""
    curr_fst = first_head
    curr_snd = second_head

    while curr_snd:
        tmp = curr_fst.next
        tmp2 = curr_snd.next
        curr_snd.next = curr_fst.next
        curr_fst.next = curr_snd
        curr_fst = tmp
        curr_snd = tmp2

    return first_head


def zip_linked_list(linked_list):
    """Zip the linked list in place."""

    length = get_linked_list_length(linked_list)

    if length <= 2:
        return linked_list

    # Split the linked list into two halves
    middle = length // 2 if length % 2 == 1 else length // 2 + 1

    second_half_head = linked_list

    for _ in range(middle):
        second_half_head = second_half_head.next
    # Reverse the second half of the linked list
    second_half_head = reverse_linked_list(second_half_head)

    # Merge the two halves
    head = merge_linked_lists(linked_list, second_half_head)

    # Set the end of the linkedlist to point to nothing
    curr = head
    for _ in range(length):
        curr = curr.next

    curr.next = None

    return head
