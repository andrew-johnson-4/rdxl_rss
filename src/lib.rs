use rdxl::{xtype,xrender};

xtype!(
   <!Rss xml_version:String={{"1.0".to_string()}} xml_encoding:String={{"UTF-8".to_string()}} version:String={{"2.0".to_string()}}>
     <!RssChannel>
       <!RssChannelTitle title:String/>
       <!RssChannelDescription description:String/>
       <!RssChannelLink link:String/>
       <!RssChannelLastBuildDate date:String/>
       <!RssChannelPubDate date:String/>
       <!RssChannelTTL ttl:u64/>
       <!RssItem>
         <!RssItemTitle title:String/>
         <!RssItemDescription description:String/>
         <!RssItemLink link:String/>
         <!RssItemGuid is_permalink:bool={{false}} guid:String/>
         <!RssItemPubDate date:String/>
       </RssItem>
     </RssChannel>
   </Rss>
);

xrender!(Rss,
   {{format!(r#"<?xml version="{}" encoding="{}" ?>"#, self.xml_version, self.xml_encoding)}}
   <rss version={{self.version}}>
     {{ for c in self.children.iter() {{
        {{ let RssChildren::RssChannel(c) = c; }}
        <channel>
          {{ for rc in c.children.iter() {{
            {{ if let RssChannelChildren::RssChannelTitle(t) = rc {{
              <title>{{ t.title }}</title>
            }} else if let RssChannelChildren::RssChannelDescription(t) = rc {{
              <description>{{ t.description }}</description>
            }} else if let RssChannelChildren::RssChannelLink(t) = rc {{
              <link>{{ t.link }}</link>
            }} else if let RssChannelChildren::RssChannelLastBuildDate(t) = rc {{
              <lastBuildDate>{{ t.date }}</lastBuildDate>
            }} else if let RssChannelChildren::RssChannelPubDate(t) = rc {{
              <pubDate>{{ t.date }}</pubDate>
            }} else if let RssChannelChildren::RssChannelTTL(t) = rc {{
              <ttl>{{ t.ttl }}</ttl>
            }} else if let RssChannelChildren::RssItem(t) = rc {{
              <item>
                {{ for ic in t.children.iter() {{
                  {{ if let RssItemChildren::RssItemTitle(c) = ic {{
                    <title>{{ c.title }}</title>
                  }} else if let RssItemChildren::RssItemDescription(c) = ic {{
                    <description>{{ c.description }}</description>
                  }} else if let RssItemChildren::RssItemLink(c) = ic {{
                    <link>{{ c.link }}</link>
                  }} else if let RssItemChildren::RssItemGuid(c) = ic {{
                    <guid isPermaLink={{ c.is_permalink }}>{{ c.guid }}</guid>
                  }} else if let RssItemChildren::RssItemPubDate(c) = ic {{
                    <pubDate>{{ c.date }}</pubDate>
                  }} }}
                }} }}
              </item>
            }} }}
          }} }}
        </channel>
     }} }}
   </rss>
);
