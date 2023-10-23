class LinkedList:
    def __init__(self, value):
        self.value = value
        self.next = None


def mergeLinkedLists(headOne, headTwo):
    # Create a dummy node to represent the merged list
    dummy_head = LinkedList(0)
    curr = dummy_head

    while headOne and headTwo:
        if headOne.value <= headTwo.value:
            curr.next = headOne
            headOne = headOne.next
        else:
            curr.next = headTwo
            headTwo = headTwo.next

        curr = curr.next

    # Attach any remaining elements from both lists
    curr.next = headOne if headOne else headTwo

    # Return the actual head of the merged list (skip the dummy head)
    return dummy_head.next

    ##
    # O(n+m)time | O(1) space - where n is the number of nodes in the first Linked list
    # and m is the number of nodes in the second Linked List
