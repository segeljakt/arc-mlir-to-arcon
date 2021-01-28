def leaky_relu(x, slope=0.01):
    return x if x >= 0 else x * slope

def handle_element(x):
    return [leaky_relu(x)]
