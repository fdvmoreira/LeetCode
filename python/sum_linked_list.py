
class LinkedList:
    def __init__(self, value):
        self.value = value
        self.next = None


def get_linked_list_digits(linked_list):
    curr = linked_list
    digits = []

    while curr:
        digits.append(curr.value)
        curr = curr.next

    return digits


def sumOfLinkedLists(linkedListOne, linkedListTwo):
    # Get the digits from lists and reverse them
    digits_list_one = get_linked_list_digits(linkedListOne)[::-1]
    digits_list_two = get_linked_list_digits(linkedListTwo)[::-1]

    num_one = int(''.join(map(str, digits_list_one)))
    num_two = int(''.join(map(str, digits_list_two)))

    # Add the numbers, convert to string, to list and then reverse it
    result = str(num_one + num_two)[::-1]

    new_linked_list = LinkedList(None)
    curr = new_linked_list

    for value in result:
        curr.next = LinkedList(int(value))
        curr = curr.next

    return new_linked_list.next


# O(max(n, m)) time | O(max(n, m)) space - where n is the length of the first Linked List and m is the length of the second Linked List
#
# -----------------------------------------------------------
# - Traverse the lists and store the value of each node
# - Reverse the stored values
# - Add them
# - Reverse the result
# - Create new lisked list and store each digit as node
#
