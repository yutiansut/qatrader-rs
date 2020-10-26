import click



@click.command()
@click.option('--account_cookie')
@click.option('--password')
@click.option('--broker')
@click.option('--wsuri', default='ws://127.0.0.1:7988')
@click.option('--eventmq_ip', default='127.0.0.1')
@click.option('--database_ip',default='127.0.0.1')
def gen_file(account_cookie, password, broker, wsuri, eventmq_ip, database_ip):
    with open(f"{account_cookie}.toml", "w") as w:
        w.write(f"[common]\n\
account= \"{account_cookie}\"\n\
password= \"{password}\"\n\
broker= \"{broker}\"\n\
wsuri= \"{wsuri}\"\n\
eventmq_ip=\"{eventmq_ip}\"\n\
database_ip=\"{database_ip}\"\n\
ping_gap=5\n\
taskid=\"\"\n\
portfolio=\"default\"\n\
bank_password=\"\"\n\
capital_password=\"\"\n\
appid=\"\"\n\
log_level=\"debug\"")



if __name__ == "__main__":
    gen_file()