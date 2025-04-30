import smtplib
from email.message import EmailMessage

# Configuration
SMTP_SERVER = 'smtp.gmail.com'
SMTP_PORT = 587
SENDER_EMAIL = 'your_email@gmail.com'
SENDER_PASSWORD = 'your_app_password'  # Use app password if 2FA is enabled

# List of recipient emails
recipient_emails = [
    'tejasvicwt@gmail.com',
    'tejasvi18we@gmail.com',
    'rupali111247@gmail.com'
]

# Email content
subject = "System Generated Message"
body = """
Hello,
This is an automated system-generated message,to upload the death certificate of your benifactor, Please do not reply.
Best regards,  
william.xyz
"""

# Create the email message
def create_email(to_email):
    msg = EmailMessage()
    msg['Subject'] = subject
    msg['From'] = SENDER_EMAIL
    msg['To'] = to_email
    msg.set_content(body)
    return msg

# Send emails
def send_bulk_emails():
    try:
        server = smtplib.SMTP(SMTP_SERVER, SMTP_PORT)
        server.starttls()
        server.login(SENDER_EMAIL, SENDER_PASSWORD)
        print("[+] Logged in successfully.")

        for email in recipient_emails:
            msg = create_email(email)
            server.send_message(msg)
            print(f"[✓] Email sent to {email}")

        server.quit()
        print("[+] All emails sent successfully.")
    except Exception as e:
        print(f"[!] Error: {e}")

if __name__ == "__main__":
    send_bulk_emails()
