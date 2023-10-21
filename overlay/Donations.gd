extends VBoxContainer


func _ready():
	$HTTPRequest.request_completed.connect(_on_request_completed)
	
	

func start():
	var headers = ["Authorization: Bearer " + get_parent().token]
	$HTTPRequest.request("https://v5api.tiltify.com/api/public/team_campaigns/1a9afc5d-1eb2-4ed3-b508-bd4f2907e496/donations", headers)

func _on_request_completed(result, response_code, headers, body):
	var json = JSON.parse_string(body.get_string_from_utf8())
	
	for donation in json["data"]:
		var donation_text = Label.new()
		donation_text.text = donation["donor_name"] + ": $" + donation["amount"]["value"]
		self.add_child(donation_text)
	
