CREATE TABLE categoies ( -- 分类
	id SERIAL PRIMARY KEY, -- 自增主键
	name VARCHAR(20) NOT NULL UNIQUE, -- 分类名称
	is_del BOOLEAN NOT NULL DEFAULT FALSE -- 是否删除
);

CREATE TABLE articles ( -- 文章
	id SERIAL PRIMARY KEY, -- 自增主键
	category_id INT NOT NULL REFERENCES categoies(id), -- 文章所属分类的ID，外键
	title VARCHAR(255) NOT NULL, -- 文章标题
	content TEXT NOT NULL, -- 文章内容
	dateline TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 添加时间
	is_del BOOLEAN NOT NULL DEFAULT FALSE -- 是否删除
);

-- 插入示例数据
INSERT INTO categoies (id,name) VALUES
(1,'Rust'), (2,'Go'), (3,'Javascript');

-- 插入更多示例数据
ALTER SEQUENCE categoies_id_seq RESTART WITH 4;
INSERT INTO categoies (name) VALUES
('U3A0CsWdiy'),
('SWACTQFa0Y'),
('GYqfhaKJ6J'),
('0sjsXVArdZ'),
('MiN8lR1g9B'),
('oBorPeyIvH'),
('cqS4jGnmxG'),
('dc0qqvbDNP'),
('jq8K6LgUFy'),
('K1tKtlvzgf'),
('Z5kEYZYEdp'),
('y3K6ryqRMF'),
('hwPu60bq1u'),
('2Idzt9CmAV'),
('vbLGfMJNHz'),
('6tTPkRtpWB'),
('sWBfrpOAIB'),
('zgmXGcYsGt'),
('WH2EBpojIS'),
('m1rsNTknqS');

-- 插入文章的示例数据
INSERT INTO articles (category_id, title,content) VALUES
(1, '标题-GLKUSroPOR', '内容-GLKUSroPOR'),
(1, '标题-hFQRulHJAk', '内容-hFQRulHJAk'),
(2, '标题-pM0TURxhwC', '内容-pM0TURxhwC'),
(1, '标题-svNJmWaqRo', '内容-svNJmWaqRo'),
(3, '标题-8XWiTUSfhB', '内容-8XWiTUSfhB'),
(2, '标题-yvwE32TLkg', '内容-yvwE32TLkg');

CREATE TABLE tags ( -- 标签
	id SERIAL PRIMARY KEY, -- 自增主键
	name VARCHAR(20) NOT NULL UNIQUE -- 标签名称
);
INSERT INTO tags (name) VALUES 
('标签1'),
('标签2'),
('标签3');

CREATE TABLE article_tags ( -- 文章标签
	id SERIAL PRIMARY KEY, -- 自增主键
	article_id INT NOT NULL REFERENCES articles(id),
	tag_id INT NOT NULL REFERENCES articles(id)
);
INSERT INTO article_tags(article_id,tag_id) VALUES 
(1, 1),
(2, 1),
(2, 2),
(3, 1),
(1, 3);
