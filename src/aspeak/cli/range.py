class Range:
    # pylint: disable=line-too-long
    """
    A range of values for argparse.
    reference: https://stackoverflow.com/questions/12116685/how-can-i-require-my-python-scripts-argument-to-be-a-float-in-a-range-using-arg
    """

    def __init__(self, start, end):
        self.start = start
        self.end = end

    def __eq__(self, other):
        return self.start <= other <= self.end

    def __repr__(self):
        return f'values in range {self.start}-{self.end} (inclusive)'
