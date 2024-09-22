import tkinter as tk
from tkinter import messagebox
import subprocess
import sys


def save_input():
    system = sys.platform
    site_value = site.get()
    username_value = username.get()
    max_value = max_text.get()
    special_value = special_val.get()
    match special_value:
        case 1:
            special_value = "yes"
        case 0:
            special_value = "no"

    match system:
        case "linux":
            subprocess.run(["./passgen", site_value, username_value, max_value, special_value])
            messagebox.showinfo("Information", "Password Generated!")
        case "win32":
            subprocess.run(["passgen1.exe", site_value, username_value, max_value, special_value])
            messagebox.showinfo("Information", "Password Generated!")
        case _:
            print("Unsupported OS!")


root = tk.Tk()
root.title("Hello world")
root.geometry("500x200")

site = tk.Entry(root, width=30)
site.place(x=20, y=20)
site_label = tk.Label(root, text="Site or Application")
site_label.place(x=280, y=20)

username = tk.Entry(root, width=30)
username.place(x=20, y=70)
username_label = tk.Label(root, text="Username or Email Address")
username_label.place(x=280, y=70)

max_text = tk.Entry(root, width=3)
max_text.place(x=20, y=120)
max_label = tk.Label(root, text="Password Length")
max_label.place(x=60, y=120)

special_val = tk.IntVar()
special = tk.Checkbutton(root, text="Special Chars?", variable=special_val)
special.place(x=280, y=120)


button = tk.Button(root, text="Generate Password", command=save_input)
button.place(x=180, y=160)

root.mainloop()
