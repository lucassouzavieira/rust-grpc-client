# Make commands
proto-get:
	curl https://raw.githubusercontent.com/lucassouzavieira/go-grpc-server/main/internal/protobuf/schema/fleet.proto --output proto/fleet.proto
	curl https://raw.githubusercontent.com/lucassouzavieira/go-grpc-server/main/internal/protobuf/schema/incident.proto --output proto/incident.proto