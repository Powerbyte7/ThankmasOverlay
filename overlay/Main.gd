extends Control


# Called when the node enters the scene tree for the first time.
#func _ready():
#	$AuthRequest.request_completed.connect(_on_request_completed)
#	const headers = ["Content-Type: application/json"]
#	var payload = "{\"client_id\":\"9ead1ff1b59360ae5b2d963563218f373b479b42f76b49b0eee3775bd76b412b\",\"client_secret\":\"bd6b2e18f36766aa557088be7e2511509d9f0024af84903228afc63f80840c31\", \"grant_type\": \"client_credentials\", \"scope\": \"public\"}"
#	$AuthRequest.request("https://v5api.tiltify.com/oauth/token", headers, HTTPClient.METHOD_POST, payload)

@export var url = "http://127.0.0.1:1338"

var socket = WebSocketPeer.new()
var running = false
var token

func _ready():
#	$AuthRequest.request_completed.connect(_on_request_completed)
#	const headers = ["Content-Type: application/json"]
#	var payload = "{\"client_id\":\"9ead1ff1b59360ae5b2d963563218f373b479b42f76b49b0eee3775bd76b412b\",\"client_secret\":\"bd6b2e18f36766aa557088be7e2511509d9f0024af84903228afc63f80840c31\", \"grant_type\": \"client_credentials\", \"scope\": \"public\"}"
#	$AuthRequest.request("https://v5api.tiltify.com/oauth/token", headers, HTTPClient.METHOD_POST, payload)

#	$AuthRequest.request_completed.connect(_on_request_completed)
#	const headers = ["Content-Type: application/json"]
#	var payload = "{\"client_id\":\"9ead1ff1b59360ae5b2d963563218f373b479b42f76b49b0eee3775bd76b412b\",\"client_secret\":\"bd6b2e18f36766aa557088be7e2511509d9f0024af84903228afc63f80840c31\", \"grant_type\": \"client_credentials\", \"scope\": \"public\"}"
#	$AuthRequest.request("https://v5api.tiltify.com/oauth/token", headers, HTTPClient.METHOD_POST, payload)
	
	$HTTPRequest.request_completed.connect(_webhook_start)
	const headers = ["Content-Type: application/json"]
	const payload = '{"user_id": 1}'
	$HTTPRequest.request(url + "/register", headers, HTTPClient.METHOD_POST, payload)
	print("lol")
	
	

func _webhook_start(result, response_code, headers, body):
	var json = JSON.parse_string(body.get_string_from_utf8())
	var error = socket.connect_to_url(json["url"])
	print(error)
	running = true

func _process(delta):
	if running:
		socket.poll()
		var state = socket.get_ready_state()
		if state == WebSocketPeer.STATE_OPEN:
			while socket.get_available_packet_count():
				var packet = socket.get_packet()
				var text = packet.get_string_from_utf8()
				$ProgressBar.update(float(text),float(10000))
				print("Packet: ", text)
		elif state == WebSocketPeer.STATE_CLOSING:
			# Keep polling to achieve proper close.
			pass
		elif state == WebSocketPeer.STATE_CLOSED:
			var code = socket.get_close_code()
			var reason = socket.get_close_reason()
			print("WebSocket closed with code: %d, reason %s. Clean: %s" % [code, reason, code != -1])
			set_process(false) # Stop processing.





# Called every frame. 'delta' is the elapsed time since the previous frame.
func _on_request_completed(result, response_code, headers, body):
	var json = JSON.parse_string(body.get_string_from_utf8())
	token = json["access_token"]
	print(token)

	$ProgressBar.start()
	$Donations.start()