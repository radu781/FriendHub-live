# FriendHub-live

A microservice designed to add chatting and notifications functionality to the FriendHub project using a socketio connection.

For instant messaging events this server is self sufficient, but for notifications, an event handler is used in the backend that redirects events here (such as post likes, comments on your posts) using gRPC.
