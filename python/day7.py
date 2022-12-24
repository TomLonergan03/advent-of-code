class Node:
    def __init__(self, name: str, parent,  value: int = None) -> None:
        self.name = name
        self.parent = parent
        self.value = value
        self.children = []

    def addChild(self, name: str, parent,  value: int = None):
        self.children.append(Node(name, parent, value))


def buildTree():
    global number_of_nodes
    file = open("../inputs/day7.txt")

    root = Node("root", None)
    current_node = root
    number_of_nodes = 0

    for line in file:
        line = line.removesuffix("\n")
        print(line)
        if "$ cd" in line:
            if ".." in line:
                current_node = current_node.parent
                continue

            line = line[5:]

            child_exists = False
            for child in current_node.children:
                if child.name == line:
                    print("Node exists: " + child.name)
                    current_node = child
                    child_exists = True
                    break
            if child_exists:
                continue

            print("Adding dir " + line + " to node " + current_node.name)
            current_node.addChild(line, current_node)
            number_of_nodes += 1
            # print(current_node.children[len(current_node.children)-1].name)
            current_node = current_node.children[len(current_node.children)-1]
            # print(current_node.name)
            continue

        if "$ ls" in line:
            continue

        if "dir " in line:
            continue

        (size, name) = line.split()
        print("Adding file " + name + " to node " + current_node.name)
        current_node.addChild(name, current_node, int(size))
        number_of_nodes += 1

    return root


def count_size(node: Node) -> int:
    global nodes_visited, number_of_nodes
    nodes_visited += 1

    if node.value == None:
        total = 0
        for child in node.children:
            total += count_size(child)
        return total

    return node.value


def count_size_for_all_nodes(node: Node) -> int:
    global nodes_visited, number_of_nodes
    nodes_visited += 1
    if node.value == None:
        total = 0
        size = count_size(node)
        if size <= 100000:
            total += size

        for child in node.children:
            total += count_size_for_all_nodes(child)

        return total

    return 0


def find_smallest_over(node, min_size):
    global nodes_visited, number_of_nodes, all_big_enough
    nodes_visited += 1

    if node.value == None:
        size = count_size(node)
        if size >= min_size:
            all_big_enough.append(size)
        for child in node.children:
            find_smallest_over(child, min_size)
        return

    return


root = buildTree()
print("\nTree built with " + str(number_of_nodes) + " nodes")
nodes_visited = 0
print("Part 1 answer = " + str(count_size_for_all_nodes(root)))
root_size = count_size(root)
print("Total size = " + str(root_size) + ", so " +
      str(30000000 - 70000000 + root_size) + " must be freed")
all_big_enough = []
find_smallest_over(root, 30000000 - 70000000 + root_size)
print("Part 2 answer = " +
      str(min(all_big_enough)))
print("Visited " + str(nodes_visited) + " nodes")
