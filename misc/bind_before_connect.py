import socket

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

s.bind(("192.168.1.162", 12345))
s.connect(("46b.it", 22))

while 1:
  data = s.recv(1024)
  print(data)
  if not data: break
  s.sendall(data)
