extends ProgressBar

var position_tween

func _ready():
	position_tween = get_tree().create_tween().set_trans(Tween.TRANS_CUBIC)
	$HTTPRequest.request_completed.connect(_on_request_completed)
	
func _input(event):
	if event.is_action_pressed("ui_down"):
		print("DOWN")
		var tween = create_tween().set_trans(Tween.TRANS_CUBIC)
		tween.tween_property(self, "position", Vector2(36,34), 1.8)
	elif event.is_action_pressed("ui_up"):
		print("UP")
		var tween = create_tween().set_trans(Tween.TRANS_CUBIC)
		tween.tween_property(self, "position", Vector2(36,-182), 1.8)
	
func start():
	
	var headers = ["Authorization: Bearer " + get_parent().token]
	$HTTPRequest.request("https://v5api.tiltify.com/api/public/team_campaigns/1a9afc5d-1eb2-4ed3-b508-bd4f2907e496/", headers)

func _on_request_completed(result, response_code, headers, body):
	var json = JSON.parse_string(body.get_string_from_utf8())
	var raised = json["data"]["amount_raised"]["value"]
	var goal = json["data"]["goal"]["value"]
	update(raised,float(goal))


func update(raised,goal):
	self.max_value = float(goal)
	$ProgressText.update(float(raised))
	
	
	
	var tween = get_tree().create_tween().set_trans(Tween.TRANS_CUBIC)
	tween.tween_property(self, "value", float(raised), 0.5)
	tween.play()
	
