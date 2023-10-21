extends LineEdit


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass


func _on_text_submitted(new_text):
	var new_raised = float(self.text)
	var tween = get_tree().create_tween().set_trans(Tween.TRANS_CUBIC)
	tween.tween_property($"../ProgressBar", "value", new_raised, 1)
	$"../ProgressBar/ProgressText".update(new_raised)
	tween.play()
