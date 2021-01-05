# coding: utf-8
import requests
res = requests.get("http://thongtinnhansu.viettel.vn/TTNS/organizationAction.do?_vt=3b24d302dd5c0a19dbe0cd443deb3406&key=165317")
res.json()
res.text
cookies = {"JSESSIONID": "52F0ABF48747A7EA3248A64CE2092114"}
res = requests.get("http://thongtinnhansu.viettel.vn/TTNS/organizationAction.do?_vt=3b24d302dd5c0a19dbe0cd443deb3406&key=165317", cookies=cookies)
res.text
url = "http://thongtinnhansu.viettel.vn/TTNS/organizationAction.do?_vt=3b24d302dd5c0a19dbe0cd443deb3406&key={key}&checkbox=true&select=false&_=1592898176539"
start_key = 165317
root = {}
requests.get(url.format(key=key), cookies=cookies).text
requests.get(url.format(key=9008631), cookies=cookies).text
def recursive_req(key, data={}):
    res = requests.get(url.format(key=key), cookies=cookies)
    child_data = []
    for row in res.json():
        child_data.append({'key': key, 'title': row['title'], "child_data": recurive_req(row['key'])})
    data['child_data'] = child_data
    return data
    
data = {}
recursive_req(start_key, data)
def recursive_req(key, data={}):
    res = requests.get(url.format(key=key), cookies=cookies)
    child_data = []
    for row in res.json():
        child_data.append({'key': key, 'title': row['title'], "child_data": recursive_req(row['key'])})
    data['child_data'] = child_data
    return data
    
recursive_req(start_key, data)
def recursive_req(key, data={}):
    res = requests.get(url.format(key=key), cookies=cookies)
    child_data = []
    for row in res.json():
        child_data.append({'key': key, 'title': row['title'], "child_data": recursive_req(row['key'])})
    data['child_data'] = child_data
    print(f'key={key}, len_child={len(child_data)}')
    return data
    
recursive_req(start_key, data)
session = requests.Session(cookies=cookies)
def recursive_req(key, data={}):
    res = session.get(url.format(key=key), cookies=cookies)
    child_data = []
    for row in res.json():
        child_data.append({'key': key, 'title': row['title'], "child_data": recursive_req(row['key'])})
    data['child_data'] = child_data
    print(f'key={key}, len_child={len(child_data)}')
    return data
    
data = {}
recursive_req(9004482, data)
session = requests.Session()
recursive_req(9004482, data)
data
len(data['child_data'])
list(data['child_data'].keys())
[a['title'] for a in data['child_data']]
for row in data:
    row['title'] = html.unescape(row['title'])
    
import html
for row in data:
    row['title'] = html.unescape(row['title'])
    
for row in data:
    try:
        row['title'] = html.unescape(row['title'])
    except:
        pass
        
    
data
def recursive_req(key, data={}):
    res = session.get(url.format(key=key), cookies=cookies)
    child_data = []
    for row in res.json():
        child_data.append({'key': key, 'title': html.unescape(row['title']), "child_data": recursive_req(row['key'])})
    data['child_data'] = child_data
    print(f'key={key}, len_child={len(child_data)}')
    return data
    
data = {}
recursive_req(9004482, data)
json.dump(data, fp=open("org_vcc.json", "w"))
import json
json.dump(data, fp=open("org_vcc.json", "w"))
data
data[0]
data['child_data']
data['child_data'][0]
data['child_data'][0]['child_data]
data['child_data'][0]['child_data']
data['child_data'][0]['child_data'][0]
data['child_data'][0]['child_data']
data['child_data'][0]['child_data']
data['child_data'][0]['child_data']
data['child_data'][0]['child_data']['child_data']
data['child_data'][0]['child_data']['child_data'][0]
data['child_data'][0]['child_data']['child_data'][0]['child_data']
data['child_data'][0]['child_data']['child_data'][0]['child_data'][0]
def recursive_req(key, data={}):
    res = session.get(url.format(key=key), cookies=cookies)
    child_data = []
    for row in res.json():
        child_data.append({'key': key, 'title': html.unescape(row['title']), "child_data": recursive_req(row['key'])})
    data['child_data'] = child_data
    print(f'key={key}, len_child={len(child_data)}')
    return child_data
    
    
data = {}recursive_req(9004482, data)
data = {}
recursive_req(9004482, data)
data
json.dump(data, fp=open("org_vcc.json", "w"))
get_ipython().system('sl .')
get_ipython().system('subl .')
data['key']= 9004482
data['title'] = "Tổng công ty Cổ phần Công trình Viettel"
json.dump(data, fp=open("org_vcc.json", "w"))
get_ipython().run_line_magic('clear', '')
get_ipython().run_line_magic('ll', '')
get_ipython().run_line_magic('ll', '')
get_ipython().run_line_magic('save', 'code.py')
get_ipython().run_line_magic('save', '')
get_ipython().run_line_magic('save', 'code')
get_ipython().run_line_magic('save', 'a.py')
get_ipython().run_line_magic('save', 'my_useful_session a')
get_ipython().run_line_magic('save', '-r ')
get_ipython().run_line_magic('save', '-r a.py')
get_ipython().run_line_magic('save', 'fr a.py 1-100')
