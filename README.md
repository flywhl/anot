# anot
Extract annotations from source-code comments.

# Usage

```
# file.py

class Something(Experiment):
    # @note: this experiment will be re-written later
    
    def run(self):
        ...

        x = 5  # @hypothesis: 5 is better than 4

        ...
```

```
$ anot file.py --treesitter --tags hypothesis,note,todo --yaml

annotations:
  - kind: note
    content: this experiment will be re-written later
    context:
      node_type: class_definition
      parent_type: module
      associated_name: Something
      line_range: [2, 7]
  - kind: hypothesis
    content: 5 is better than 4
    context:
      node_type: assignment
      parent_type: function_definition
      associated_name: run
      line_number: 5
      variable_name: x
```
