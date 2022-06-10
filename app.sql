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
