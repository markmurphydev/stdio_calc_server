Client program parses postfix 4-function calculator strings into an AST.

Client starts a new server process, communicating over stdio.

Client serializes to Postcard format, sends AST to server,
and server evaluates and responds with the result.

<img width="665" alt="スクリーンショット 2025-03-04 午後11 48 04" src="https://github.com/user-attachments/assets/bc532106-73eb-45bd-aab9-ab3438e750f6" />
