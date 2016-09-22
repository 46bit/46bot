//s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
//# Let the source address be 192.168.1.21:1234
//s.bind(("192.168.1.21", 1234))
//s.connect(("www.google.com", 80))

#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <netdb.h>

int main(int argc, char *argv[]) {
   int portno, clilen;
   char buffer[256];
   struct sockaddr_in serv_addr, cli_addr;

   int fd = socket(AF_INET, SOCK_STREAM, 0);
   if (fd < 0) {
      perror("ERROR opening socket");
      exit(1);
   }

   if (setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &(int){1}, sizeof(int)) < 0) {
      perror("ERROR on setsockopt SO_REUSEADDR");
      exit(1);
   }

   ////////////////////////////////////////////////////////////////////////////////

   /* Initialize socket structure */
   bzero((char *) &cli_addr, sizeof(cli_addr));
   portno = 5344;

   cli_addr.sin_family = AF_INET;
   cli_addr.sin_addr.s_addr = inet_addr("192.168.1.162");
   cli_addr.sin_port = htons(portno);

   // Now bind the host address using bind() call.
   if (bind(fd, (struct sockaddr *) &cli_addr, sizeof(cli_addr)) < 0) {
      perror("ERROR on binding");
      exit(1);
   }

   ////////////////////////////////////////////////////////////////////////////////

   // Initialize socket structure
   bzero((char *) &serv_addr, sizeof(serv_addr));
   portno = 22;

   serv_addr.sin_family = AF_INET;
   serv_addr.sin_addr.s_addr = inet_addr("104.236.192.168");
   serv_addr.sin_port = htons(portno);

   if (connect(fd, (struct sockaddr *) &serv_addr, sizeof(serv_addr)) < 0) {
      perror("ERROR on connecting");
      exit(1);
   }

   printf("Successful.\n");
}

/*
int main(void) {
  int s = socket(socket.AF_INET, socket.SOCK_STREAM)
  # Let the source address be 192.168.1.21:1234
  bind(("192.168.1.21", 1234))
  connect(("www.google.com", 80))

  bind(fd, )

}

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
# Let the source address be 192.168.1.21:1234
s.bind(("192.168.1.162", 12345))
s.connect(("46b.it", 22))
while 1:
  data = s.recv(1024)
  print(data)
  if not data: break
  s.sendall(data)
*/

/*
#include <sys/types.h>
#include <sys/socket.h>

weak! {
    int bind(int sockfd, const struct sockaddr *addr, socklen_t addrlen);
    int connect(int sockfd, const struct sockaddr *addr, socklen_t addrlen);
    fn accept4(c_int, *mut sockaddr, *mut socklen_t, c_int) -> c_int
}
if let Some(accept) = accept4.get() {
    let res = cvt_r(|| unsafe {
        accept(self.0.raw(), storage, len, SOCK_CLOEXEC)
    });
    match res {
        Ok(fd) => return Ok(Socket(FileDesc::new(fd))),
        Err(ref e) if e.raw_os_error() == Some(libc::ENOSYS) => {}
        Err(e) => return Err(e),
    }
}
*/
